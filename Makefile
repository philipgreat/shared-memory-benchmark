all:
	cargo run --release -- writer &
	cargo run --release -- reader
kill:
	pkill -INT cargo || true
	sleep 2
	pkill -9 cargo || true