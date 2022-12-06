use calamine::DataType;
use chrono::*;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub tran_date: Option<NaiveDate>,
    pub tran_time: Option<NaiveTime>,
    pub value_date: Option<NaiveDate>,
    pub tran_id: String,
    pub tran_particular: String,
    pub tran_remarks: String,
    pub ref_num: String,
    pub stan: String,
    pub term_id: String,
    pub dr_cr: String,
    pub cr: f64,
    pub dr: f64,
    pub outstanding: f64,
}

impl InputAccount {
    pub fn new(line: &[DataType]) -> Self {
        InputAccount {
            tran_date: NaiveDate::parse_from_str(line[0].to_string().trim(), "%d-%b-%y").ok(),
            tran_time: NaiveTime::parse_from_str(line[1].to_string().trim(), "%H:%M:%S").ok(),
            value_date: NaiveDate::parse_from_str(line[3].to_string().trim(), "%d-%b-%y").ok(),
            tran_id: line[4].to_string().trim().to_string(),
            tran_particular: line[5].to_string().trim().to_string(),
            tran_remarks: line[6].to_string().trim().to_string(),
            ref_num: line[7].to_string().trim().to_string(),
            stan: line[8].to_string().trim().to_string(),
            term_id: line[9].to_string().trim().to_string(),
            dr_cr: line[10].to_string().trim().to_string(),
            cr: line[11].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            dr: line[12].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            outstanding: line[13].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
        }
    }
}
