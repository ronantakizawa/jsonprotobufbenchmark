mod test_data;
mod benchmark;

use benchmark::PerformanceTester;
use clap::{Parser, ArgAction};
use colored::*;
use tokio;

#[derive(Parser, Debug)]
#[command(
    name = "protobuf-json-benchmark",
    version = "1.0.0",
    about = "Benchmarks JSON vs Protocol Buffers performance",
    long_about = "A comprehensive benchmark tool that compares JSON and Protocol Buffers across multiple performance dimensions."
)]
struct Args {
    /// Size of the test data (number of elements)
    #[arg(short, long, default_value_t = 20)]
    size: usize,
    
    /// Number of iterations for each test
    #[arg(short, long, default_value_t = 1000)]
    iterations: usize,
    
    /// Run a specific test only
    #[arg(short, long)]
    test: Option<String>,
    
    /// Enable verbose output
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

// Entry point of the application - regular main function
fn main() {
    // Use tokio runtime without the macro
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async_main());
}

// Async main function that will be run inside the tokio runtime
async fn async_main() {
    // Parse command line arguments
    let args = Args::parse();
    
    println!("{}", "JSON vs Protocol Buffers Benchmark".green().bold());
    println!("=====================================");
    println!("Data size: {}", args.size);
    println!("Iterations: {}", args.iterations);
    println!();
    
    // Create a tester instance
    let mut tester = PerformanceTester::new(args.size, args.iterations);
    
    // If a specific test is requested, run only that test
    if let Some(test_name) = args.test {
        match test_name.as_str() {
            "serialization" => {
                let result = tester.test_serialization_speed();
                println!("JSON: {:.4} ms", result.json);
                println!("Protobuf: {:.4} ms", result.protobuf);
                println!("Winner: {}", result.winner);
            },
            "deserialization" => {
                let result = tester.test_deserialization_speed();
                println!("JSON: {:.4} ms", result.json);
                println!("Protobuf: {:.4} ms", result.protobuf);
                println!("Winner: {}", result.winner);
            },
            "payload" => {
                let result = tester.test_payload_size();
                println!("JSON uncompressed: {} bytes", result.uncompressed.json);
                println!("Protobuf uncompressed: {} bytes", result.uncompressed.protobuf);
                println!("JSON compressed: {} bytes", result.compressed.json);
                println!("Protobuf compressed: {} bytes", result.compressed.protobuf);
                println!("Uncompressed winner: {}", result.uncompressed.winner);
                println!("Compressed winner: {}", result.compressed.winner);
            },
            "cpu" => {
                let result = tester.test_cpu_usage();
                println!("JSON: {:.2} ms", result.json);
                println!("Protobuf: {:.2} ms", result.protobuf);
                println!("Winner: {}", result.winner);
            },
            "memory" => {
                let result = tester.test_memory_usage();
                println!("JSON: {:.2} ms", result.json);
                println!("Protobuf: {:.2} ms", result.protobuf);
                println!("Winner: {}", result.winner);
            },
            "network" => {
                let result = tester.test_network_transfer().await;
                println!("JSON: {:.2} ms", result.json);
                println!("Protobuf: {:.2} ms", result.protobuf);
                println!("Winner: {}", result.winner);
            },
            "latency" => {
                let result = tester.test_latency_under_load().await;
                println!("JSON: {:.2} ms", result.json);
                println!("Protobuf: {:.2} ms", result.protobuf);
                println!("Winner: {}", result.winner);
            },
            "init" => {
                let result = tester.test_parser_initialization();
                println!("JSON: {:.2} ms", result.json);
                println!("Protobuf: {:.2} ms", result.protobuf);
                println!("Winner: {}", result.winner);
            },
            "throughput" => {
                let result = tester.test_throughput();
                println!("JSON: {:.2} ops/s", result.json);
                println!("Protobuf: {:.2} ops/s", result.protobuf);
                println!("Winner: {}", result.winner);
            },
            "schema" => {
                let result = tester.test_schema_evolution();
                println!("JSON: {:.4} ms", result.json);
                println!("Protobuf backwards: {:.4} ms", result.protobuf_backwards);
                println!("Protobuf forwards: {:.4} ms", result.protobuf_forwards);
                println!("Protobuf average: {:.4} ms", result.protobuf_average);
                println!("Winner: {}", result.winner);
            },
            _ => {
                println!("Unknown test: {}", test_name);
                println!("Available tests: serialization, deserialization, payload, cpu, memory, network, latency, init, throughput, schema");
            }
        }
    } else {
        // Run all tests and print results
        let _results = tester.run_all_tests().await;
        
        // Print table of results
        tester.print_results();
    }
}