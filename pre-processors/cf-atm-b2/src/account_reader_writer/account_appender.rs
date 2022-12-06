use account_reader_writer::account_reader::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use rbdate::*;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.gl_date = if let Some(dt) = acc.gl_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.tran_id = acc.tran_id;
    out_acc.ref_num = acc.ref_num;
    out_acc.particulars = acc.particulars;
    out_acc.tran_dr_amt = acc.tran_dr_amt;
    out_acc.tran_cr_amt = acc.tran_cr_amt;
    out_acc.ccy_code = acc.ccy_code;
    out_acc.balance = acc.balance;
    out_acc.remarks = acc.remarks;
    out_acc.branch = acc.branch;

    out_acc.amount = if acc.tran_dr_amt != 0.0 {
        acc.tran_dr_amt
    } else {
        acc.tran_cr_amt
    };

    out_acc
}
