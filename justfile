test day:
  cargo nextest run -- {{day}}

bench day:
  cargo bench --bench {{day}}

watch day:
  cargo watch -s "just test {{day}}" -s "just bench {{day}}"

run feature:
  cargo run --features {{feature}}
