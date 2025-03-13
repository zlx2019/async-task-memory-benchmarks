benchmark: build-examples
	./target/release/examples/benchmark_rust_tokio 1000000
	./target/release/examples/benchmark_rust_async_std 1000000
	./target/release/examples/benchmark_rust_tokio_futures 1000000
#	./target/release/examples/benchmark_rust_thread 1000000
	

build-examples:
	cargo build --examples --release 
