test:
  cargo nextest run

watch-all-test:
  cargo watch -x "nextest run"

watch-test part:
  cargo watch -x "nextest run -- {{part}}"

run feature:
  cargo run --features {{feature}}
