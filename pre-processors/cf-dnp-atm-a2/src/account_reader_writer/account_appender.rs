use account_reader_writer::account_reader::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use chrono::NaiveTime;
use rbdate::*;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.cb_tran_no = acc.cb_tran_no;
    out_acc.time = acc.time;
    out_acc.description = acc.description;
    out_acc.account_number = acc.account_number;
    out_acc.amount = acc.amount;
    out_acc.ledger_code = acc.ledger_code;
    out_acc.date = if let Some(dt) = acc.date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    if let Some(date) = acc.date {
        let time = NaiveTime::from_num_seconds_from_midnight(acc.time as u32, 0);
        let datetime = NaiveDateTime::new(date, time);
        out_acc.datetime = datetime.timestamp();
    }

    out_acc
}
