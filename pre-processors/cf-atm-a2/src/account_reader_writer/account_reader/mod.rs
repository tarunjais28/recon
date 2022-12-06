use calamine::DataType;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub datetime: String,
    pub ref_no: i64,
    pub crd_no: String,
    pub acc: String,
    pub txn: String,
    pub disp: String,
    pub extra: String,
    pub resp: i64,
    pub status: String,
}

impl InputAccount {
    pub fn new(line: &[DataType]) -> Self {
        InputAccount {
            datetime: line[0].to_string().trim().to_string(),
            ref_no: line[1].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            crd_no: line[2].to_string().trim().to_string(),
            acc: line[3].to_string().trim().to_string(),
            txn: line[4].to_string().trim().to_string(),
            disp: line[5].to_string().trim().to_string(),
            extra: line[6].to_string().trim().to_string(),
            resp: line[7].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            status: line[8].to_string().trim().to_string(),
        }
    }

    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.datetime,
            self.ref_no,
            self.crd_no,
            self.acc,
            self.txn,
            self.disp,
            self.extra,
            self.resp,
            self.status,
        )
    }
}
