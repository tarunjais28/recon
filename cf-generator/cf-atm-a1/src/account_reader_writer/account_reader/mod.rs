use calamine::DataType;
use rbdate::*;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub tran_date: Option<NaiveDate>,
    pub tran_time: String,
    pub value_date: Option<NaiveDate>,
    pub tran_id: String,
    pub tran_particular: String,
    pub tran_remarks: i64,
    pub ref_num: i64,
    pub stan: i64,
    pub term_id: i64,
    pub debit_credit: String,
    pub cr: f64,
    pub dr: f64,
    pub outstanding: f64,
    pub customer_account: i64,
}

impl InputAccount {
    pub fn new(line: &[DataType]) -> Self {
        InputAccount {
            tran_date: NaiveDate::parse_from_str(&line[0].to_string().trim(), "%d-%b-%y").ok(),
            tran_time: line[1].to_string().trim().to_string(),
            value_date: NaiveDate::parse_from_str(&line[3].to_string().trim(), "%d-%b-%y").ok(),
            tran_id: line[4].to_string().trim().to_string(),
            tran_particular: line[5].to_string().trim().to_string(),
            tran_remarks: line[6].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            ref_num: line[7].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            stan: line[8].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            term_id: line[9].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            debit_credit: line[10].to_string().trim().to_string(),
            cr: line[11].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            dr: line[12].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            outstanding: line[13].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            customer_account: line[14].to_string().trim().parse().unwrap_or(DEFAULT_INT),
        }
    }
}
