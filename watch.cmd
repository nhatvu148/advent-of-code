set MIMALLOC_LARGE_OS_PAGES=1
set MIMALLOC_PAGE_RESET=0
set RUST_BACKTRACE=1
cargo watch --clear --delay 0 ^
    -x "run --bin %*"
    @REM -- python run.py %*
    @REM -x "test"