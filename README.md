# JSON vs Protocol Buffers Benchmark

A comprehensive Rust benchmarking tool to compare the performance of JSON and Protocol Buffers (protobuf) across multiple dimensions.

## Overview

This benchmark suite rigorously tests JSON and Protocol Buffers in 11 key performance areas:

1. **Serialization Speed**: Time to convert in-memory objects to wire format
2. **Deserialization Speed**: Time to parse wire format back to objects
3. **Payload Size**: Raw byte size comparison (uncompressed)
4. **Compressed Size**: Size after gzip compression
5. **CPU Usage**: Processing overhead
6. **Memory Usage**: Memory allocation requirements
7. **Network Transfer**: Simulated transfer time over network
8. **Latency Under Load**: Performance under concurrent operations
9. **Parser Initialization**: Startup time
10. **Throughput**: Operations per second
11. **Schema Evolution**: Handling of schema/format changes

## Installation

### Prerequisites

- Rust and Cargo (1.56.0 or newer)
- Protocol Buffers compiler (`protoc`)

### Installing Protocol Buffers

**Ubuntu/Debian**:
```bash
sudo apt install -y protobuf-compiler
```

**macOS**:
```bash
brew install protobuf
```

**Windows**:
Download from https://github.com/protocolbuffers/protobuf/releases

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/protobuf-json-benchmark.git
   cd protobuf-json-benchmark
   ```

2. Create necessary directories:
   ```bash
   mkdir -p src/generated proto
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Running All Benchmarks

```bash
cargo run --release
```

This will execute all benchmark tests with the default settings and display a comparison table.

### Running a Specific Test

```bash
cargo run --release -- --test serialization
```

Available test options:
- `serialization`
- `deserialization`
- `payload`
- `cpu`
- `memory`
- `network`
- `latency`
- `init`
- `throughput`
- `schema`

### Customizing Tests

```bash
cargo run --release -- --size 100 --iterations 5000
```

Options:
- `--size` or `-s`: Number of elements in test data (default: 20)
- `--iterations` or `-i`: Number of iterations for each test (default: 1000)
- `--verbose` or `-v`: Enable verbose output

## Sample Results

```
+--------------------------+----------+-----------------------+------------+----------+
| Test                     | JSON     | Protobuf              | Difference | Winner   |
+--------------------------+----------+-----------------------+------------+----------+
| Serialization (ms/op)    | 0.0040   | 0.0021                | 188.75%    | Protobuf |
| Deserialization (ms/op)  | 0.0156   | 0.0129                | 121.42%    | Protobuf |
| Payload Size (bytes)     | 2008     | 1120                  | 179.29%    | Protobuf |
| Compressed Size (bytes)  | 445      | 352                   | 126.42%    | Protobuf |
| CPU Usage (ms)           | 122.84   | 149.93                | 81.93%     | JSON     |
| Memory Usage (proxy ms)  | 12.88    | 15.33                 | 83.99%     | JSON     |
| Network Transfer (ms)    | 51.53    | 50.85                 | 101.33%    | Protobuf |
| Latency Under Load (ms)  | 23.23    | 23.23                 | 99.98%     | JSON     |
| Parser Init (ms)         | 0.01     | 5.00                  | 0.20%      | JSON     |
| Throughput (ops/s)       | 84467.99 | 66680.06              | 126.68%    | JSON     |
| Schema Evolution (ms/op) | 0.0351   | B: 0.0139 / F: 0.0145 | 247.85%    | Protobuf |
+--------------------------+----------+-----------------------+------------+----------+
Overall winner: Protocol Buffers (6 wins vs 5 wins)
```

## Key Insights

### When to Use Protocol Buffers
- When network bandwidth is constrained
- For maximum serialization/deserialization efficiency
- When message size matters (mobile apps, IoT)
- For services with well-defined, evolving schemas
- In high-scale microservice architectures

### When to Use JSON
- When human readability is important
- When CPU or memory usage is the bottleneck
- For quick prototyping without schema definition
- When startup time is critical (serverless, short-lived processes)
- For browser compatibility without additional libraries

## Project Structure

```
protobuf-json-benchmark/
├── Cargo.toml        - Project configuration
├── src/
│   ├── main.rs       - CLI and entry point
│   ├── test_data.rs  - Test data generation
│   ├── benchmark.rs  - Benchmark implementations
│   └── generated/    - Generated protobuf code
├── proto/
│   ├── person.proto         - Original schema
│   └── person_evolved.proto - Schema with additional fields
└── build.rs          - Build script for protobuf compilation
```

## Implementation Details

The benchmark uses:
- [prost](https://github.com/tokio-rs/prost) for Protocol Buffers
- [serde_json](https://github.com/serde-rs/json) for JSON
- [tokio](https://github.com/tokio-rs/tokio) for async operations
- [flate2](https://github.com/rust-lang/flate2-rs) for compression tests
- [clap](https://github.com/clap-rs/clap) for command-line argument parsing
- [prettytable-rs](https://github.com/phsym/prettytable-rs) for results display

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.