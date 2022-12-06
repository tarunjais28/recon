use calamine::DataType;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub gl_date: Option<NaiveDate>,
    pub tran_id: String,
    pub ref_num: String,
    pub particulars: String,
    pub tran_dr_amt: f64,
    pub tran_cr_amt: f64,
    pub ccy_code: String,
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
            tran_dr_amt: line[6].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            tran_cr_amt: line[7].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            ccy_code: line[8].to_string().trim().to_string(),
            balance: line[9].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            remarks: line[10].to_string().trim().to_string(),
            branch: line[11].to_string().trim().to_string(),
        }
    }

    pub fn print_header(&self) -> String {
        format!(
            "gl_date|tran_id|ref_num|particulars|tran_dr_amt|tran_cr_amt|ccy_code|balance|remarks|branch\n"
        )
    }

    pub fn print(&self) -> String {
        format!(
            "{:?}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.gl_date,
            self.tran_id,
            self.ref_num,
            self.particulars,
            self.tran_dr_amt,
            self.tran_cr_amt,
            self.ccy_code,
            self.balance,
            self.remarks,
            self.branch,
        )
    }
}
