use agg_rules::rule::Rule;
use agg_rules::rule::RuleBuilder;
use agg_rules::rule::RuleYieldDescriptor;
use agg_rules::rule_step::Comparator;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::Split;

mod rule;
mod rule_list;
mod rule_step;

#[derive(Debug)]
pub struct AggRules {
    ord_rules: Vec<Rule>,
}

impl AggRules {
    pub fn new_from_path(path: &str, reader: &Reader) -> AggRules {
        return AggRulesBuilder::new_from_path(path, reader);
    }

    pub fn llg_for_acc(
        &self,
        account1: &AccountWithCFs,
        account2: &AccountWithCFs,
    ) -> Result<RuleYieldDescriptor, String> {
        for rule in &self.ord_rules {
            let rule_status = rule.does_acc_abide(account1, account2);
            match rule_status {
                Ok(rule_yield_descr) => return Ok(rule_yield_descr),
                Err(failed_cond) => return Err(failed_cond),
            }
        }
        return Err("No rules configured!!".to_string());
    }
}

#[derive(Debug)]
struct AggRulesBuilder {
    rule_builders: Vec<RuleBuilder>,
}

impl AggRulesBuilder {
    fn new_from_path(path: &str, reader: &Reader) -> AggRules {
        let rdr = new_buf_rdr(path).expect("File not present");
        let mut rules = AggRulesBuilder {
            rule_builders: vec![],
        };
        for line in rdr.lines() {
            rules.build_with_intermediate_rule_step(IntermediateRuleStep::new_from_line(
                line.unwrap(),
                reader,
            ));
        }

        return rules.build();
    }

    fn build_with_intermediate_rule_step(&mut self, step: IntermediateRuleStep) {
        // Very naive, potentially n^3 complexity being achieved here.
        // But it's acceptable for the moment because:
        //  a) We want to shave off development speed
        //  b) These methods will be called once at app start-up
        //  c) Computers love vectors.
        //
        // A HashMap can be used for O(1) lookup and insertion in vec, if performance has taken a significant hit...
        for rule in self.rule_builders.iter_mut() {
            if rule.rule_id == step.rule_id {
                rule.build_with(step);
                return;
            }
        }

        let rule = RuleBuilder::new_with_first(step);
        self.rule_builders.push(rule);
    }

    fn build(self) -> AggRules {
        let mut ord_rules = Vec::new();

        for rule_builder in self.rule_builders {
            ord_rules.push(rule_builder.build())
        }

        ord_rules.sort_by(|curr, next| {
            curr.sequence
                .partial_cmp(&next.sequence)
                .expect("Couldn't compare?")
        });

        return AggRules { ord_rules };
    }
}

#[derive(Debug)]
pub(crate) struct IntermediateRuleStep {
    rule_id: i32,
    rule_sequence: i32,
    rule_step: i32,
    field_name1: String,
    field_name2: String,
    comparator: Comparator,
    connector: Connector,
}

impl IntermediateRuleStep {
    fn new_from_line(line: String, reader: &Reader) -> Self {
        let mut val_iter = line.split('|');
        let rule_id = val_iter
            .next()
            .expect("Could not read `rule_id`.")
            .parse()
            .expect("Could not parse `rule_id`.");
        let rule_ord = val_iter
            .next()
            .expect("Could not read `rule_ord`.")
            .parse()
            .expect("Could not parse `rule_ord`.");
        let rule_step = val_iter
            .next()
            .expect("Could not read `rule_step`.")
            .parse()
            .expect("Could not parse `rule_step`.");

        let field_name1 = val_iter
            .next()
            .expect("Could not read `field_name1`.")
            .to_string();
        let field_type = match reader.get_field_type(&field_name1) {
            Some(typ) => typ,
            None => {
                panic!("Could not determine type of first field `{}`", field_name1);
            }
        };
        let comparator = Comparator::new_from_iter(&mut val_iter, field_type);
        let field_name2 = val_iter
            .next()
            .expect("Could not read `field_name2`.")
            .to_string();
        let connector = Connector::new_from_iter(&mut val_iter);

        return IntermediateRuleStep {
            rule_id,
            rule_sequence: rule_ord,
            rule_step,
            field_name1,
            field_name2,
            comparator,
            connector,
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Connector {
    And,
    Or,
    End(i32),
}

impl Connector {
    fn new_from_iter(iter: &mut Split<char>) -> Self {
        let connector_str = iter.next().expect("Could not read `Connector`.");
        match connector_str {
            "AND" => Connector::And,
            "OR" => Connector::Or,
            _ => {
                // If we can parse as an integer, it's the LLG
                let llg_opt: Result<i32, ParseIntError> = connector_str.parse();
                if let Ok(llg) = llg_opt {
                    return Connector::End(llg);
                } else {
                    panic!("Connector value not recognised");
                }
            }
        }
    }
}
