NUM_JOBS := 5
START := 3000
END := 4000

python:
	@echo "Running Python Fibonnaci in Parallel"
	@time seq $(START) $(END) | parallel -j $(NUM_JOBS) python fib.py
	@echo

rust:
	@echo "Running Rust Fibonnaci in Parallel"
	@cargo build --release --quiet
	@time seq $(START) $(END) | parallel -j $(NUM_JOBS) ./target/release/fib
	@cargo clean

all: python rust