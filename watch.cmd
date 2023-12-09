set MIMALLOC_LARGE_OS_PAGES=1
set MIMALLOC_PAGE_RESET=0
set RUST_BACKTRACE=1
@REM cargo watch --clear --delay 0 -- python run.py %*
cargo watch --clear --delay 0 -x "run --bin %*"
@REM cargo watch --clear --delay 0 -x "test"