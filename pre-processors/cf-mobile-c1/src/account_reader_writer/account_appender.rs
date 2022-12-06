use account_reader_writer::account_reader::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use chrono::*;
use rbdate::timestamp;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.tran_date = if let Some(dt) = acc.tran_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.tran_time = if let Some(time) = acc.tran_time {
        (time.hour() * 3600 + time.minute() * 60 + time.second()) as i64
    } else {
        DEFAULT_INT
    };
    out_acc.value_date = if let Some(dt) = acc.value_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.tran_id = acc.tran_id;
    out_acc.tran_particular = acc.tran_particular;
    out_acc.tran_remarks = acc.tran_remarks;
    out_acc.ref_num = acc.ref_num;
    out_acc.stan = acc.stan;
    out_acc.term_id = acc.term_id;
    out_acc.dr_cr = acc.dr_cr;
    out_acc.cr = acc.cr;
    out_acc.dr = acc.dr;
    out_acc.outstanding = acc.outstanding;
    out_acc.tran_datetime = out_acc.tran_date + out_acc.tran_time;
    out_acc.amount = if out_acc.dr_cr.to_uppercase() == "C" {
        out_acc.cr
    } else if out_acc.dr_cr.to_uppercase() == "D" {
        out_acc.dr
    } else {
        DEFAULT_FLOAT
    };
    out_acc.ccy = String::from("KES");
    let other_party_info: Vec<&str> = out_acc.tran_particular.split(' ').collect();
    out_acc.other_party_info = other_party_info.last().unwrap_or(&"NA").to_string();

    out_acc
}
