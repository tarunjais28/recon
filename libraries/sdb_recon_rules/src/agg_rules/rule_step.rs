use math::round;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use std::str::Split;

#[derive(Debug)]
pub(crate) struct RuleStep {
    pub(crate) field_name1: String,
    pub(crate) field_name2: String,
    pub(crate) comparator: Comparator,
}

impl RuleStep {
    pub fn does_acc_abide(&self, account1: &AccountWithCFs, account2: &AccountWithCFs) -> bool {
        match &self.comparator {
            Comparator::EQ(expected_val) => match expected_val {
                Value::I32 => {
                    return &account1.get_i32_for_key(&self.field_name1).unwrap_or(0)
                        == &account2.get_i32_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F32 => {
                    return &account1.get_f32_for_key(&self.field_name1).unwrap_or(0.0)
                        == &account2.get_f32_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::I64 => {
                    return &account1.get_i64_for_key(&self.field_name1).unwrap_or(0)
                        == &account2.get_i64_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F64 => {
                    return &account1.get_f64_for_key(&self.field_name1).unwrap_or(0.0)
                        == &account2.get_f64_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::String => {
                    return account1
                        .get_string_for_key(&self.field_name1)
                        .unwrap_or(&"NA".to_string())
                        == account2
                            .get_string_for_key(&self.field_name2)
                            .unwrap_or(&"NA".to_string());
                }
            },
            Comparator::REQ(expected_val, precision) => match expected_val {
                Value::I32 => {
                    return &account1.get_i32_for_key(&self.field_name1).unwrap_or(0)
                        == &account2.get_i32_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F32 => {
                    let num_1 = account1.get_f32_for_key(&self.field_name1).unwrap_or(0.0) as f64;
                    let num_2 = account2.get_f32_for_key(&self.field_name2).unwrap_or(0.0) as f64;
                    return round::stochastic(num_1, *precision)
                        == round::stochastic(num_2, *precision);
                }
                Value::I64 => {
                    return &account1.get_i64_for_key(&self.field_name1).unwrap_or(0)
                        == &account2.get_i64_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F64 => {
                    let num_1 = account1.get_f64_for_key(&self.field_name1).unwrap_or(0.0);
                    let num_2 = account2.get_f64_for_key(&self.field_name2).unwrap_or(0.0);
                    return round::stochastic(num_1, *precision)
                        == round::stochastic(num_2, *precision);
                }
                Value::String => {
                    return account1
                        .get_string_for_key(&self.field_name1)
                        .unwrap_or(&"NA".to_string())
                        == account2
                            .get_string_for_key(&self.field_name2)
                            .unwrap_or(&"NA".to_string());
                }
            },
            Comparator::AEQ(expected_val, precision) => match expected_val {
                Value::I32 => {
                    return &account1
                        .get_i32_for_key(&self.field_name1)
                        .unwrap_or(0)
                        .abs()
                        == &account2
                            .get_i32_for_key(&self.field_name2)
                            .unwrap_or(0)
                            .abs();
                }
                Value::F32 => {
                    let num_1 = account1.get_f32_for_key(&self.field_name1).unwrap_or(0.0) as f64;
                    let num_2 = account2.get_f32_for_key(&self.field_name2).unwrap_or(0.0) as f64;
                    return round::stochastic(num_1, *precision).abs()
                        == round::stochastic(num_2, *precision).abs();
                }
                Value::I64 => {
                    return &account1
                        .get_i64_for_key(&self.field_name1)
                        .unwrap_or(0)
                        .abs()
                        == &account2
                            .get_i64_for_key(&self.field_name2)
                            .unwrap_or(0)
                            .abs();
                }
                Value::F64 => {
                    let num_1 = account1.get_f64_for_key(&self.field_name1).unwrap_or(0.0);
                    let num_2 = account2.get_f64_for_key(&self.field_name2).unwrap_or(0.0);
                    return round::stochastic(num_1, *precision).abs()
                        == round::stochastic(num_2, *precision).abs();
                }
                Value::String => {
                    return account1
                        .get_string_for_key(&self.field_name1)
                        .unwrap_or(&"NA".to_string())
                        == account2
                            .get_string_for_key(&self.field_name2)
                            .unwrap_or(&"NA".to_string());
                }
            },
            Comparator::TEQ(expected_val, precision) => match expected_val {
                Value::I64 => {
                    let threshold_time_in_secs = match precision {
                        1 => 1,      // 1 sec
                        2 => 60,     // 1 min
                        3 => 3600,   // 1 hour
                        4 => 86400,  // 1 day
                        5 => 604800, // 1 week
                        _ => panic!("Invalid precision with Time Equality Comparator!!"),
                    };
                    let time_diff = account1.get_i64_for_key(&self.field_name1).unwrap_or(0)
                        - account2.get_i64_for_key(&self.field_name2).unwrap_or(0);
                    if time_diff.abs() < threshold_time_in_secs {
                        true
                    } else {
                        false
                    }
                }
                _ => panic!("Invalid field used with Time Equality Comparator!!"),
            },
            Comparator::NEQ(expected_val) => match expected_val {
                Value::I32 => {
                    return &account1.get_i32_for_key(&self.field_name1).unwrap_or(0)
                        != &account2.get_i32_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F32 => {
                    return &account1.get_f32_for_key(&self.field_name1).unwrap_or(0.0)
                        != &account2.get_f32_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::I64 => {
                    return &account1.get_i64_for_key(&self.field_name1).unwrap_or(0)
                        != &account2.get_i64_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F64 => {
                    return &account1.get_f64_for_key(&self.field_name1).unwrap_or(0.0)
                        != &account2.get_f64_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::String => {
                    return account1
                        .get_string_for_key(&self.field_name1)
                        .unwrap_or(&"NA".to_string())
                        != account2
                            .get_string_for_key(&self.field_name2)
                            .unwrap_or(&"NA".to_string());
                }
            },
            Comparator::RNEQ(expected_val, precision) => match expected_val {
                Value::I32 => {
                    return &account1.get_i32_for_key(&self.field_name1).unwrap_or(0)
                        != &account2.get_i32_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F32 => {
                    let num_1 = account1.get_f32_for_key(&self.field_name1).unwrap_or(0.0) as f64;
                    let num_2 = account2.get_f32_for_key(&self.field_name2).unwrap_or(0.0) as f64;
                    return round::stochastic(num_1, *precision)
                        != round::stochastic(num_2, *precision);
                }
                Value::I64 => {
                    return &account1.get_i64_for_key(&self.field_name1).unwrap_or(0)
                        != &account2.get_i64_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F64 => {
                    let num_1 = account1.get_f64_for_key(&self.field_name1).unwrap_or(0.0);
                    let num_2 = account2.get_f64_for_key(&self.field_name2).unwrap_or(0.0);
                    return round::stochastic(num_1, *precision)
                        != round::stochastic(num_2, *precision);
                }
                Value::String => {
                    return account1
                        .get_string_for_key(&self.field_name1)
                        .unwrap_or(&"NA".to_string())
                        != account2
                            .get_string_for_key(&self.field_name2)
                            .unwrap_or(&"NA".to_string());
                }
            },
            Comparator::ANEQ(expected_val, precision) => match expected_val {
                Value::I32 => {
                    return &account1
                        .get_i32_for_key(&self.field_name1)
                        .unwrap_or(0)
                        .abs()
                        != &account2
                            .get_i32_for_key(&self.field_name2)
                            .unwrap_or(0)
                            .abs();
                }
                Value::F32 => {
                    let num_1 = account1.get_f32_for_key(&self.field_name1).unwrap_or(0.0) as f64;
                    let num_2 = account2.get_f32_for_key(&self.field_name2).unwrap_or(0.0) as f64;
                    return round::stochastic(num_1, *precision).abs()
                        != round::stochastic(num_2, *precision).abs();
                }
                Value::I64 => {
                    return &account1
                        .get_i64_for_key(&self.field_name1)
                        .unwrap_or(0)
                        .abs()
                        != &account2
                            .get_i64_for_key(&self.field_name2)
                            .unwrap_or(0)
                            .abs();
                }
                Value::F64 => {
                    let num_1 = account1.get_f64_for_key(&self.field_name1).unwrap_or(0.0);
                    let num_2 = account2.get_f64_for_key(&self.field_name2).unwrap_or(0.0);
                    return round::stochastic(num_1, *precision).abs()
                        != round::stochastic(num_2, *precision).abs();
                }
                Value::String => {
                    return account1
                        .get_string_for_key(&self.field_name1)
                        .unwrap_or(&"NA".to_string())
                        != account2
                            .get_string_for_key(&self.field_name2)
                            .unwrap_or(&"NA".to_string());
                }
            },
            Comparator::LT(expected_val) => match expected_val {
                Value::I32 => {
                    return &account1.get_i32_for_key(&self.field_name1).unwrap_or(0)
                        > &account2.get_i32_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F32 => {
                    return &account1.get_f32_for_key(&self.field_name1).unwrap_or(0.0)
                        > &account2.get_f32_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::I64 => {
                    return &account1.get_i64_for_key(&self.field_name1).unwrap_or(0)
                        > &account2.get_i64_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F64 => {
                    return &account1.get_f64_for_key(&self.field_name1).unwrap_or(0.0)
                        > &account2.get_f64_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::String => {
                    return account1
                        .get_string_for_key(&self.field_name1)
                        .unwrap_or(&"NA".to_string())
                        > account2
                            .get_string_for_key(&self.field_name2)
                            .unwrap_or(&"NA".to_string());
                }
            },
            Comparator::LTEQ(expected_val) => match expected_val {
                Value::I32 => {
                    return &account1.get_i32_for_key(&self.field_name1).unwrap_or(0)
                        >= &account2.get_i32_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F32 => {
                    return &account1.get_f32_for_key(&self.field_name1).unwrap_or(0.0)
                        >= &account2.get_f32_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::I64 => {
                    return &account1.get_i64_for_key(&self.field_name1).unwrap_or(0)
                        >= &account2.get_i64_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F64 => {
                    return &account1.get_f64_for_key(&self.field_name1).unwrap_or(0.0)
                        >= &account2.get_f64_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::String => {
                    return account1
                        .get_string_for_key(&self.field_name1)
                        .unwrap_or(&"NA".to_string())
                        >= account2
                            .get_string_for_key(&self.field_name2)
                            .unwrap_or(&"NA".to_string());
                }
            },
            Comparator::GT(expected_val) => match expected_val {
                Value::I32 => {
                    return &account1.get_i32_for_key(&self.field_name1).unwrap_or(0)
                        < &account2.get_i32_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F32 => {
                    return &account1.get_f32_for_key(&self.field_name1).unwrap_or(0.0)
                        < &account2.get_f32_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::I64 => {
                    return &account1.get_i64_for_key(&self.field_name1).unwrap_or(0)
                        < &account2.get_i64_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F64 => {
                    return &account1.get_f64_for_key(&self.field_name1).unwrap_or(0.0)
                        < &account2.get_f64_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::String => {
                    return account1
                        .get_string_for_key(&self.field_name1)
                        .unwrap_or(&"NA".to_string())
                        < account2
                            .get_string_for_key(&self.field_name2)
                            .unwrap_or(&"NA".to_string());
                }
            },
            Comparator::GTEQ(expected_val) => match expected_val {
                Value::I32 => {
                    return &account1.get_i32_for_key(&self.field_name1).unwrap_or(0)
                        <= &account2.get_i32_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F32 => {
                    return &account1.get_f32_for_key(&self.field_name1).unwrap_or(0.0)
                        <= &account2.get_f32_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::I64 => {
                    return &account1.get_i64_for_key(&self.field_name1).unwrap_or(0)
                        <= &account2.get_i64_for_key(&self.field_name2).unwrap_or(0);
                }
                Value::F64 => {
                    return &account1.get_f64_for_key(&self.field_name1).unwrap_or(0.0)
                        <= &account2.get_f64_for_key(&self.field_name2).unwrap_or(0.0);
                }
                Value::String => {
                    return account1
                        .get_string_for_key(&self.field_name1)
                        .unwrap_or(&"NA".to_string())
                        <= account2
                            .get_string_for_key(&self.field_name2)
                            .unwrap_or(&"NA".to_string());
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum Comparator {
    EQ(Value),
    REQ(Value, i8),
    AEQ(Value, i8),
    TEQ(Value, i8),
    NEQ(Value),
    RNEQ(Value, i8),
    ANEQ(Value, i8),
    LT(Value),
    LTEQ(Value),
    GT(Value),
    GTEQ(Value),
}

impl Comparator {
    pub fn new_from_iter(iter: &mut Split<char>, typ: reader::types::Type) -> Self {
        let comparator_str = iter.next().expect("Could not read `Comparator`.");
        let mut alpha_codes: Vec<&str> = comparator_str.split(|c: char| c.is_numeric()).collect();
        alpha_codes.retain(|&x| x != "");
        let mut num_codes: Vec<&str> = comparator_str.split(|c: char| c.is_alphabetic()).collect();
        num_codes.retain(|&x| x != "");
        let comparator = alpha_codes[0];
        match comparator {
            "EQ" => {
                return Comparator::EQ(Comparator::get_next_as_val(typ));
            }
            "REQ" => {
                let mut precision = 1;
                if num_codes.len() != 0 {
                    precision = num_codes[0].parse().unwrap_or(1);
                }
                return Comparator::REQ(Comparator::get_next_as_val(typ), precision);
            }
            "AEQ" => {
                let mut precision = 1;
                if num_codes.len() != 0 {
                    precision = num_codes[0].parse().unwrap_or(1);
                }
                return Comparator::AEQ(Comparator::get_next_as_val(typ), precision);
            }
            "TEQ" => {
                let mut precision = 1;
                if num_codes.len() != 0 {
                    precision = num_codes[0].parse().unwrap_or(1);
                }
                return Comparator::TEQ(Comparator::get_next_as_val(typ), precision);
            }
            "LT" => {
                return Comparator::LT(Comparator::get_next_as_val(typ));
            }
            "GT" => {
                return Comparator::GT(Comparator::get_next_as_val(typ));
            }
            "NEQ" => {
                return Comparator::NEQ(Comparator::get_next_as_val(typ));
            }
            "RNEQ" => {
                let mut precision = 1;
                if num_codes.len() != 0 {
                    precision = num_codes[0].parse().unwrap_or(1);
                }
                return Comparator::RNEQ(Comparator::get_next_as_val(typ), precision);
            }
            "ANEQ" => {
                let mut precision = 1;
                if num_codes.len() != 0 {
                    precision = num_codes[0].parse().unwrap_or(1);
                }
                return Comparator::ANEQ(Comparator::get_next_as_val(typ), precision);
            }
            "GTEQ" => {
                return Comparator::GTEQ(Comparator::get_next_as_val(typ));
            }
            "LTEQ" => {
                return Comparator::LTEQ(Comparator::get_next_as_val(typ));
            }
            _ => {
                println!("Unexpected Comparator Found: {}", comparator_str);
                panic!("Terminating program due to unexpected comparator!!");
            }
        }
    }

    fn get_next_as_val(typ: reader::types::Type) -> Value {
        match typ {
            reader::types::Type::I32 => {
                return Value::I32;
            }
            reader::types::Type::F32 => {
                return Value::F32;
            }
            reader::types::Type::I64 => {
                return Value::I64;
            }
            reader::types::Type::F64 => {
                return Value::F64;
            }
            reader::types::Type::String => {
                return Value::String;
            }
            reader::types::Type::Cashflows => {
                panic!("Rule engine does not support `Cashflows` type!!");
            }
        }
    }
}

#[derive(Debug)]
pub enum Value {
    I32,
    F32,
    I64,
    F64,
    String,
}
