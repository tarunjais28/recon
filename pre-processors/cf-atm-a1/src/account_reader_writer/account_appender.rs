use account_reader_writer::account_reader::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use chrono::NaiveTime;
use rbdate::*;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.tran_date = if let Some(dt) = acc.tran_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.tran_time = acc.tran_time;
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
    out_acc.debit_credit = acc.debit_credit;
    out_acc.cr = acc.cr;
    out_acc.dr = acc.dr;
    out_acc.outstanding = acc.outstanding;
    out_acc.customer_account = acc.customer_account;

    let mut cd_no = format!(
        "{:6}XXXXXX{:4}",
        acc.tran_remarks / 10_000_000_000,
        acc.tran_remarks % 10_000
    );
    cd_no = cd_no.replace(" ", "0");
    out_acc.card_no = cd_no;

    if let Some(date) = acc.tran_date {
        let hms: Vec<&str> = out_acc.tran_time.split(':').collect();
        let def_time: u32 = 0;
        let hour = hms[0].parse().unwrap_or(def_time);
        let min = hms[1].parse().unwrap_or(def_time);
        let sec = hms[2].parse().unwrap_or(def_time);
        let time = NaiveTime::from_hms(hour, min, sec);
        let datetime = NaiveDateTime::new(date, time);
        out_acc.datetime = datetime.timestamp();
    }
    out_acc.ccy = String::from("KES");

    out_acc.amount = if out_acc.debit_credit.to_uppercase() == "C" {
        out_acc.cr
    } else if out_acc.debit_credit.to_uppercase() == "D" {
        out_acc.dr
    } else {
        DEFAULT_FLOAT
    };

    out_acc
}
