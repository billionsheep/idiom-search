#!/usr/bin/env bash
set -euo pipefail

sqlite3 data/idioms.sqlite "SELECT COUNT(*) FROM idioms;"
cargo fmt --check
cargo test
cargo run -- stats
cargo run -- exact 画蛇添足
cargo run -- search 多此一举 --limit 3
cargo run -- list --rare --limit 3

echo "verify ok"
