use account_reader_writer::account_reader::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use chrono::*;
use rbdate::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.datetime = acc.datetime;
    out_acc.date = timestamp(NaiveDateTime::from_timestamp(out_acc.datetime, 0).date());
    out_acc.time = out_acc.datetime - out_acc.date;

    out_acc.row_id = acc.row_id;
    out_acc.merchant_code = acc.merchant_code;
    out_acc.service_account_id = acc.service_account_id;
    out_acc.tx_reference = acc.tx_reference;
    out_acc.tx_reference1 = acc.tx_reference1;
    out_acc.amount = acc.amount;
    out_acc.status = acc.status;
    out_acc.trx_type = acc.trx_type;
    out_acc.status_description = acc.status_description;
    out_acc.trx_source = acc.trx_source;
    out_acc.biller_status = acc.biller_status;
    out_acc.country = acc.country;
    out_acc.discount_amount = acc.discount_amount;
    out_acc.amount_txn = acc.amount_txn;
    out_acc.access_mode = acc.access_mode;

    out_acc
}
