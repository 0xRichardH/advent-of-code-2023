test-all:
  cargo nextest run

test day:
  cargo nextest run -- {{day}}

bench day:
  cargo bench --bench {{day}}

watch day:
  cargo watch -s "just test {{day}}" -s "just bench {{day}}"

flamegraph feature:
  cargo flamegraph --profile flamegraph --root --features {{feature}} -o flamegraphs/{{feature}}.svg

run feature:
  cargo run --features {{feature}}
