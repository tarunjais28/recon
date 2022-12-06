use account_reader_writer::account_reader::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.receipt_no = acc.receipt_no;
    out_acc.completion_datetime = match acc.completion_datetime {
        Some(datetime) => datetime.timestamp(),
        None => 0,
    };
    out_acc.initiation_datetime = match acc.initiation_datetime {
        Some(datetime) => datetime.timestamp(),
        None => 0,
    };
    out_acc.details = acc.details;
    out_acc.tran_status = acc.tran_status;
    out_acc.paid_in = acc.paid_in;
    out_acc.withdrawn = acc.withdrawn;
    out_acc.amount = acc.withdrawn + acc.paid_in;
    out_acc.balance = acc.balance;
    out_acc.balance_confirmed = acc.balance_confirmed;
    out_acc.reason_type = acc.reason_type;
    out_acc.other_party_info = acc.other_party_info;
    out_acc.linked_tran_id = acc.linked_tran_id;

    out_acc
}
