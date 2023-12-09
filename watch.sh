#!/usr/bin/env bash
# MIMALLOC_LARGE_OS_PAGES=1 MIMALLOC_PAGE_RESET=0 RUST_BACKTRACE=1 cargo watch --clear --delay 0 -- ./run.py "$@"
MIMALLOC_LARGE_OS_PAGES=1 MIMALLOC_PAGE_RESET=0 RUST_BACKTRACE=1 cargo watch --clear --delay 0 -x "run --bin $*"
# MIMALLOC_LARGE_OS_PAGES=1 MIMALLOC_PAGE_RESET=0 RUST_BACKTRACE=1 cargo watch --clear --delay 0 -x "test"