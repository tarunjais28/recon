use account_reader_writer::account_reader::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use rbdate::*;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    let datetime_field: Vec<&str> = acc.datetime.split(' ').collect();
    let mut date_time = String::from(datetime_field[0]);
    date_time.push_str(datetime_field[1]);
    if let Ok(datetime) = NaiveDateTime::parse_from_str(&date_time, "%d/%m/%y %H:%M:%S") {
        out_acc.datetime = datetime.timestamp();
    }
    out_acc.term_id = datetime_field[2].parse().unwrap_or(DEFAULT_INT);
    out_acc.ref_no = acc.ref_no;
    out_acc.crd_no = acc.crd_no;
    out_acc.acc = acc.acc;
    out_acc.txn = acc.txn;

    let disp: Vec<&str> = acc.disp.split(' ').collect();
    if disp.len() == 2 {
        out_acc.ccy = String::from(disp[0]);
        let mut amount = String::from(disp[1]);
        amount.retain(|c| c != ',');
        out_acc.amount = amount.parse().unwrap_or(DEFAULT_FLOAT);
    }
    out_acc.extra = acc.extra;
    out_acc.resp = acc.resp;
    out_acc.status = acc.status;

    out_acc
}
