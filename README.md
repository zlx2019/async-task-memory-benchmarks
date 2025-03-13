# How Much Memory Do You Need in 2024 to Run 1 Million Concurrent Tasks?

## Machine Configure
- OS: macOS Sequoia 15.3.1 arm64
- Kernel: Darwin 24.3.0
- CPU: Apple M4 (10)
- GPU: Apple M4 (10)
- Memory: 24.00 GiB

## Language Versions
- Rust: 1.85.0 Tokio: 1.44.0
- Java: 21
- Go: 1.24.1
- Python: 3.13.2
- NodeJs: 22.14.0
- Deno: 2.2.3

## Results 
100k task
- Rust(Tokio): 372M
- Rust(async_std): 502M
- Python: 1.3G
- Go: 2.5G
- NodeJS: 556M
- Deno: 818M
- Java(OpenJDK): 996M ~ 1.1G