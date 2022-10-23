set export := true
set positional-arguments := true
set dotenv-load := true

YEAR := '2015'
RUSTFLAGS := '-C target-cpu=native'

set-context DAY:
    echo "DAY=$1" > .env

build:
    cargo build --release

test:
    env RUST_LOG=debug cargo nextest run day$DAY::

lint:
    cargo clippy

bench:
    # requires: cargo install cargo-criterion
    cargo criterion --bench day$DAY

edit:
    nvim -O3 src/day$DAY.rs puzzle/day$DAY.md input/day$DAY.txt

@run:
    cargo run --release -- $DAY

@run-all:
    cargo run --release

@run-debug:
    env RUST_LOG=debug cargo run -- $DAY

download-input DAY:
    mkdir -p input
    [ -e "input/day$1.txt" ] || aoc download -d "$1" -y $YEAR -f "input/day$1.txt"
    chmod 444 "input/day$1.txt"

download-puzzle DAY:
    mkdir -p puzzle
    aoc -y $YEAR -d "$1" read > "puzzle/day$1.md"

template DAY:
    env DAY={{DAY}} envsubst < ./template/dayXX.rs.in | sed -E -e 's/^(\s*)0([1-9])/\1\2/' > src/day$DAY.rs
    env DAY={{DAY}} envsubst < ./template/bench_dayXX.rs.in > benches/day$DAY.rs
    grep -q "mod day{{DAY}};" src/lib.rs || echo -e "\nmod day{{DAY}};\npub use day{{DAY}}::Day{{DAY}};" >> src/lib.rs
    grep -q Day{{DAY}} src/main.rs || sed -i src/main.rs -E -e "s@\s*// marker@        Box::new(aoc{{YEAR}}::Day{{DAY}}::new()),\n        // marker@"
    echo -e "\n[[bench]]\nname = \"day$DAY\"\nharness = false" >> Cargo.toml

generate DAY: (set-context DAY) (download-puzzle DAY) (download-input DAY) (template DAY)
    git add .
    git commit -m "Day $1, Part 1: Start"
    just edit

submit PART ANSWER:
    aoc -y $YEAR -d $DAY submit {{PART}} {{ANSWER}}
    just download-puzzle $DAY
