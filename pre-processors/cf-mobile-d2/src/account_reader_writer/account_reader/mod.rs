use calamine::DataType;
use chrono::*;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub gl_date: Option<NaiveDate>,
    pub tran_id: String,
    pub ref_num: String,
    pub particulars: String,
    pub debit_amt: f64,
    pub credit_amt: f64,
    pub ccy: String,
    pub balance: f64,
    pub remarks: String,
    pub branch: String,
}

impl InputAccount {
    pub fn new(line: &[DataType]) -> Self {
        InputAccount {
            gl_date: NaiveDate::parse_from_str(line[0].to_string().trim(), "%d-%m-%y").ok(),
            tran_id: line[1].to_string().trim().to_string(),
            ref_num: line[3].to_string().trim().to_string(),
            particulars: line[5].to_string().trim().to_string(),
            debit_amt: line[6].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            credit_amt: line[7].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            ccy: line[8].to_string().trim().to_string(),
            balance: line[9].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            remarks: line[10].to_string().trim().to_string(),
            branch: line[11].to_string().trim().to_string(),
        }
    }
}
