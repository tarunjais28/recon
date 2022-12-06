#!/usr/bin/env bash

INPUT=$"test-bed/B1. CDM E-Journal.xlsx"
OUTPUT=$"test-bed/CFOutput"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
    --input-file "${INPUT}" \
    --input-sheet-name cdm \
    --output-file ${OUTPUT} \
    --log-file ${LOG_FILE} \
    --diagnostics-log-file ${DIAGNOSTICS_FILE} \
    --as-on-date 28-02-2018 \
    # --log-level trace \
    # --diagnostics-flag true
