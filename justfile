test-all:
  cargo nextest run

test day:
  cargo nextest run -- {{day}}

bench day:
  cargo bench --bench {{day}}

bench-all:
    cargo bench -q > benchmarks.txt

watch day:
  cargo watch -qcs "just test {{day}}" -s "just bench {{day}}"

flamegraph feature:
  cargo flamegraph --profile flamegraph --root --features {{feature}} -o flamegraphs/{{feature}}.svg

run feature:
  cargo run --features {{feature}}
