use slog::Logger;
mod account_appender;
mod account_reader;
mod account_without_cashflows;
mod account_writer;
mod io;

use self::account_reader::InputAccount;
use self::account_writer::AccountWithoutCashflows;
use self::io::*;
use account_reader_writer::account_appender::create_account_without_cashflows;
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::fs::File;
use std::io::BufWriter;
use std::time::SystemTime;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_prin_in_inp = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut succ_rec = DEFAULT_INT;
    let start_generator_timer = SystemTime::now();
    let (mut writer, mut unpro_writer, unpro_file_path) =
        create_io_workers(config_params.output_file_path(), log);
    let mut op_line: String = String::new();

    let mut input_file =
        open_workbook_auto(config_params.input_file_path()).expect("Unable to open input file.");
    if let Some(Ok(reader)) = input_file.worksheet_range(config_params.input_sheet_name()) {
        for row in reader.rows().skip(4) {
            let input_account = InputAccount::new(&row);
            tot_acc_encntrd += 1;
            if tot_acc_encntrd == 1 {
                op_line.push_str(&input_account.print_header());
            }
            if input_account.datetime < 0 {
                op_line.push_str(&input_account.print());
                continue;
            }
            succ_rec += 1;
            tot_prin_in_inp += input_account.amount_txn;

            let account_without_cashflows = log_measurements!(
                diag_log,
                [format!(
                    "Type: CreateAccWithCFs, Identifier: {}",
                    input_account.tx_reference
                )],
                create_account_without_cashflows(input_account)
            );
            tot_prin_in_op += account_without_cashflows.amount_txn;

            log_measurements!(
                diag_log,
                [format!(
                    "Type: WriteAccWithCFs, Identifier: {}",
                    account_without_cashflows.tx_reference
                )],
                writer.write(account_without_cashflows)
            );
        }
    }
    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    log_debug!(log, "Total Duration: {:?}", total_duration);

    output_writer(&mut unpro_writer, op_line, &unpro_file_path);

    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        succ_rec,
        tot_acc_encntrd - succ_rec,
        tot_prin_in_inp,
        tot_prin_in_op,
        0,
    );
    println!("{}", health_stat.display());
    info!(log, "{}", health_stat.display());
    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn create_io_workers(
    output_path: &str,
    log: &Logger,
) -> (AccountWithoutCashflows, BufWriter<File>, String) {
    let writer = AccountWithoutCashflows::new(output_path, log);
    let mut failed_file_path = String::from(output_path);
    failed_file_path.push_str("-unprocessed.txt");
    let unpro_writer = get_writer(&failed_file_path);

    (writer, unpro_writer, failed_file_path)
}
