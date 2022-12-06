use calamine::DataType;
use chrono::*;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub receipt_no: String,
    pub completion_datetime: Option<NaiveDateTime>,
    pub initiation_datetime: Option<NaiveDateTime>,
    pub details: i64,
    pub tran_status: String,
    pub paid_in: f64,
    pub withdrawn: f64,
    pub balance: f64,
    pub balance_confirmed: String,
    pub reason_type: String,
    pub other_party_info: String,
    pub linked_tran_id: String,
    pub ac_no: i64,
}

impl InputAccount {
    pub fn new(line: &[DataType]) -> Self {
        InputAccount {
            receipt_no: line[0].to_string().trim().to_string(),
            completion_datetime: NaiveDateTime::parse_from_str(
                line[1].to_string().trim(),
                "%d-%m-%Y %H:%M:%S",
            )
            .ok(),
            initiation_datetime: NaiveDateTime::parse_from_str(
                line[2].to_string().trim(),
                "%d-%m-%Y %H:%M:%S",
            )
            .ok(),
            details: line[3].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            tran_status: line[4].to_string().trim().to_string(),
            paid_in: line[5].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            withdrawn: line[6].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            balance: line[7].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            balance_confirmed: line[8].to_string().trim().to_string(),
            reason_type: line[9].to_string().trim().to_string(),
            other_party_info: line[10].to_string().trim().to_string(),
            linked_tran_id: line[11].to_string().trim().to_string(),
            ac_no: line[12].to_string().trim().parse().unwrap_or(DEFAULT_INT),
        }
    }
}
