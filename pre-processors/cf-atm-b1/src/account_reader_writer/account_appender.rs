use account_reader_writer::account_reader::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use chrono::*;
use rbdate::timestamp;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.s_no = acc.s_no;
    out_acc.acc_no = acc.acc_no;
    out_acc.machin = acc.machin;
    out_acc.customer_name = acc.customer_name;
    out_acc.deposit_reference = acc.deposit_reference;
    out_acc.typ = acc.typ;
    out_acc.stat = acc.stat;
    out_acc.ccy = acc.ccy;
    out_acc.amount = acc.amount;
    out_acc.txn_ref = acc.txn_ref;
    out_acc.txn_datetime = if let Some(dt) = acc.txn_date {
        out_acc.txn_date = timestamp(dt.date());
        out_acc.txn_time =
            (dt.time().hour() * 3600 + dt.time().minute() * 60 + dt.time().second()) as i64;
        dt.timestamp()
    } else {
        DEFAULT_INT
    };

    out_acc
}
