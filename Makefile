sqlite-dump:
	sqlite3 -cmd .dump files.db ""

find-duplicates:
	sqlite3 -line files.db "select size, sha256, group_concat(filename) from files group by size, sha256 having count(*) > 1 order by size"

test:
	cargo test
	cargo clippy --all-targets --all-features
