use calamine::DataType;
use chrono::NaiveDateTime;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub datetime: i64,
    pub row_id: f64,
    pub merchant_code: String,
    pub service_account_id: String,
    pub tx_reference: String,
    pub tx_reference1: String,
    pub amount: f64,
    pub status: String,
    pub trx_type: String,
    pub status_description: String,
    pub trx_source: String,
    pub biller_status: String,
    pub country: String,
    pub discount_amount: f64,
    pub amount_txn: f64,
    pub access_mode: String,
}

impl InputAccount {
    pub fn new(line: &[DataType]) -> Self {
        InputAccount {
            datetime: ((line[0].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT) - 25569.0)
                * 86400.0) as i64,
            row_id: line[1].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            merchant_code: line[2].to_string().trim().to_string(),
            service_account_id: line[3].to_string().trim().to_string(),
            tx_reference: line[4].to_string().trim().to_string(),
            tx_reference1: line[5].to_string().trim().to_string(),
            amount: line[6].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            status: line[7].to_string().trim().to_string(),
            trx_type: line[8].to_string().trim().to_string(),
            status_description: line[9].to_string().trim().to_string(),
            trx_source: line[10].to_string().trim().to_string(),
            biller_status: line[11].to_string().trim().to_string(),
            country: line[12].to_string().trim().to_string(),
            discount_amount: line[13].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            amount_txn: line[14].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            access_mode: line[15].to_string().trim().to_string(),
        }
    }

    pub fn print_header(&self) -> String {
        format!(
            "datetime|row_id|merchant_code|service_account_id|tx_reference|tx_reference1|amount|status|trx_type|status_description|trx_source|biller_status|country|discount_amount|amount_txn|access_mode\n"
        )
    }

    pub fn print(&self) -> String {
        format!(
            "{:?}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            NaiveDateTime::from_timestamp(self.datetime, 0),
            self.row_id,
            self.merchant_code,
            self.service_account_id,
            self.tx_reference,
            self.tx_reference1,
            self.amount,
            self.status,
            self.trx_type,
            self.status_description,
            self.trx_source,
            self.biller_status,
            self.country,
            self.discount_amount,
            self.amount_txn,
            self.access_mode,
        )
    }
}
