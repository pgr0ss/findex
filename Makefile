sqlite-dump:
	sqlite3 -cmd .dump files.db ""

find-duplicates:
	sqlite3 -line files.db "select size, sha256, group_concat(filename) from files group by size, sha256 having count(*) > 1 order by size"

flamegraph:
	cargo flamegraph --dev --root -- add .

test:
	cargo test
	cargo clippy --all-targets --all-features
	cargo fmt --check

update:
	rustup update stable
