build:
	# Building release version
	cargo build --release
leak_check: build
	# Leak-check using Valgrind
	valgrind --leak-check=yes ./target/release/testbed
run: build
	# Running application
	./target/release/testbed
