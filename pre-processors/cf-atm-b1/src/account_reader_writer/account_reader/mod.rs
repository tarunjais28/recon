use calamine::DataType;
use rbdate::*;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub s_no: i64,
    pub acc_no: i64,
    pub machin: String,
    pub customer_name: String,
    pub deposit_reference: String,
    pub typ: String,
    pub stat: String,
    pub ccy: String,
    pub amount: f64,
    pub txn_ref: String,
    pub txn_date: Option<NaiveDateTime>,
}

impl InputAccount {
    pub fn new(line: &[DataType]) -> Self {
        InputAccount {
            s_no: line[0].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            acc_no: line[3].to_string().trim().parse().unwrap_or(DEFAULT_INT),
            machin: line[11].to_string().trim().to_string(),
            customer_name: line[13].to_string().trim().to_string(),
            deposit_reference: line[16].to_string().trim().to_string(),
            typ: line[23].to_string().trim().to_string(),
            stat: line[25].to_string().trim().to_string(),
            ccy: line[28].to_string().trim().to_string(),
            amount: line[29].to_string().trim().parse().unwrap_or(DEFAULT_FLOAT),
            txn_ref: line[34].to_string().trim().to_string(),
            txn_date: {
                let dt = NaiveDateTime::parse_from_str(
                    line[40].to_string().trim(),
                    "%Y-%m-%d %H:%M:%S%.f",
                );
                if dt.is_err() {
                    NaiveDateTime::parse_from_str(line[40].to_string().trim(), "%Y-%m-%d %H:%M:")
                } else {
                    dt
                }
            }
            .ok(),
        }
    }

    pub fn print_header(&self) -> String {
        format!(
            "s_no|acc_no|machin|customer_name|deposit_reference|typ|stat|ccy|amount|txn_ref|txn_datetime\n"
        )
    }

    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:?}\n",
            self.s_no,
            self.acc_no,
            self.machin,
            self.customer_name,
            self.deposit_reference,
            self.typ,
            self.stat,
            self.ccy,
            self.amount,
            self.txn_ref,
            self.txn_date,
        )
    }
}
