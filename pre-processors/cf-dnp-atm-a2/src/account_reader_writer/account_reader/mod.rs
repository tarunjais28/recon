use calamine::DataType;
use rbdate::*;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub cb_tran_no: i64,
    pub date: Option<NaiveDate>,
    pub time: i64,
    pub description: String,
    pub account_number: i64,
    pub amount: f64,
    pub ledger_code: i64,
}

impl InputAccount {
    pub fn new(line: &[DataType]) -> Self {
        InputAccount {
            cb_tran_no: line[0].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            date: datevalue_to_naive_date(line[1].to_string().trim()).ok(),
            time: (line[2].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT) * 86400.0) as i64,
            description: line[3].to_string().trim().to_string(),
            account_number: line[4].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            amount: line[5].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            ledger_code: line[6].to_string().trim().parse().unwrap_or(DEFAULT_INT),
        }
    }
}
