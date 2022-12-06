#!/usr/bin/env bash

INPUT=$"test-bed/D1. C2B_Web.xlsx"
OUTPUT=$"test-bed/CFOutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
    --input-file "${INPUT}" \
    --input-sheet-name Web \
    --output-file ${OUTPUT} \
    --log-file ${LOG_FILE} \
    --diagnostics-log-file ${DIAGNOSTICS_FILE} \
    --as-on-date 18-08-2020 \
    # --log-level trace \
    # --diagnostics-flag true
