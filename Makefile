all: kill
	cargo run --release -- writer &
	cargo run --release -- reader
kill:
	pkill -INT shared-memory-benchmark || true
	sleep 2
	pkill -9 shared-memory-benchmark || true
