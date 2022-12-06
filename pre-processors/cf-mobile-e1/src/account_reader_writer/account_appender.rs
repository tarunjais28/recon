use account_reader_writer::account_reader::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.gl_date = match acc.gl_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.tran_id = acc.tran_id;
    out_acc.ref_num = acc.ref_num;
    out_acc.particulars = acc.particulars;
    out_acc.debit_amt = acc.debit_amt;
    out_acc.credit_amt = acc.credit_amt;
    out_acc.amount = acc.credit_amt + acc.debit_amt;
    out_acc.ccy = acc.ccy;
    out_acc.balance = acc.balance;
    out_acc.remarks = acc.remarks;
    out_acc.branch = acc.branch;

    out_acc
}
