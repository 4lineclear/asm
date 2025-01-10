alias r := run
alias c := cargo

run *ARGS:
    cd util/rasm && cargo build && \
    cd - > /dev/null && ./util/rasm/target/debug/rasm {{ARGS}}

cargo *ARGS:
    cd util/rasm && cargo {{ARGS}}
