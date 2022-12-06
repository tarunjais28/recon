use calamine::DataType;
use rbdate::*;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub tran_no: i64,
    pub description: String,
    pub amount: f64,
    pub date: Option<NaiveDate>,
    pub time: i64,
    pub balance: f64,
    pub account_number: i64,
    pub unwanted: String,
}

impl InputAccount {
    pub fn new(line: &[DataType]) -> Self {
        InputAccount {
            tran_no: line[0].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            description: line[1].to_string().trim().to_string(),
            amount: line[2].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            date: datevalue_to_naive_date(line[3].to_string().trim()).ok(),
            time: (line[4].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT) * 86400.0) as i64,
            balance: line[5].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            account_number: line[6].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            unwanted: line[7].to_string().trim().to_string(),
        }
    }
}
