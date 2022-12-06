use agg_rules::rule_step::Comparator;
use agg_rules::rule_step::RuleStep;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

#[derive(Debug)]
/// One set of `AND` joined RuleSteps in a Rule
pub(crate) struct RSList {
    steps: Vec<RS>,
}

impl RSList {
    pub(crate) fn new() -> Self {
        RSList { steps: Vec::new() }
    }

    pub(crate) fn add_rule_step(&mut self, rs: RS) {
        self.steps.push(rs)
    }

    pub(crate) fn does_acc_abide(
        &self,
        account1: &AccountWithCFs,
        account2: &AccountWithCFs,
    ) -> RSListAbidanceRes {
        for step in &self.steps {
            if step.does_acc_abide(account1, account2) == true {
                continue;
            } else {
                let comparator = match &step.step.comparator {
                    Comparator::EQ(_) => "EQ",
                    Comparator::REQ(_, _) => "REQ",
                    Comparator::AEQ(_, _) => "AEQ",
                    Comparator::TEQ(_, _) => "TEQ",
                    Comparator::NEQ(_) => "NEQ",
                    Comparator::RNEQ(_, _) => "RNEQ",
                    Comparator::ANEQ(_, _) => "ANEQ",
                    Comparator::LT(_) => "LT",
                    Comparator::LTEQ(_) => "LTEQ",
                    Comparator::GT(_) => "GT",
                    Comparator::GTEQ(_) => "GTEQ",
                };
                let failed_cond = format!(
                    "{} {} {}",
                    step.step.field_name1, comparator, step.step.field_name2
                );
                return RSListAbidanceRes::False(failed_cond);
            }
        }

        return RSListAbidanceRes::True;
    }
}

pub enum RSListAbidanceRes {
    False(String),
    True,
}

#[derive(Debug)]
pub(crate) struct RS {
    pub(crate) step_num: i32,
    pub(crate) step: RuleStep,
}

impl RS {
    pub fn does_acc_abide(&self, account1: &AccountWithCFs, account2: &AccountWithCFs) -> bool {
        self.step.does_acc_abide(account1, account2)
    }
}
