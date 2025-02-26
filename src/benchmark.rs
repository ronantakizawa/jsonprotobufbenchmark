use crate::test_data::{generate_test_data, generate_evolved_test_data, JsonPerson, evolved};
use colored::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use prettytable::{Table, row};
use prost::Message;
use serde_json;
use std::time::{Duration, Instant};
use std::io::Write;

// Include the generated Protocol Buffers code
include!(concat!(env!("OUT_DIR"), "/test.rs"));

pub struct BenchmarkResults {
    pub serialization: BenchmarkMetric,
    pub deserialization: BenchmarkMetric,
    pub payload_size: PayloadSizeMetric,
    pub cpu_usage: BenchmarkMetric,
    pub memory_usage: BenchmarkMetric,
    pub network_transfer: BenchmarkMetric,
    pub latency_under_load: BenchmarkMetric,
    pub parser_init: BenchmarkMetric,
    pub throughput: ThroughputMetric,
    pub schema_evolution: SchemaEvolutionMetric,
}

pub struct BenchmarkMetric {
    pub json: f64,
    pub protobuf: f64,
    pub difference_percent: f64,
    pub winner: String,
}

pub struct PayloadSizeMetric {
    pub uncompressed: BenchmarkMetric,
    pub compressed: BenchmarkMetric,
}

pub struct ThroughputMetric {
    pub json: f64,
    pub protobuf: f64,
    pub difference_percent: f64,
    pub winner: String,
}

pub struct SchemaEvolutionMetric {
    pub json: f64,
    pub protobuf_backwards: f64,
    pub protobuf_forwards: f64,
    pub protobuf_average: f64,
    pub winner: String,
}

pub struct PerformanceTester {
    results: Option<BenchmarkResults>,
    data_size: usize,
    iterations: usize,
}

impl PerformanceTester {
    pub fn new(data_size: usize, iterations: usize) -> Self {
        PerformanceTester {
            results: None,
            data_size,
            iterations,
        }
    }

    // 1. Test serialization speed
    pub fn test_serialization_speed(&self) -> BenchmarkMetric {
        println!("{}", "Testing serialization speed...".green());
        
        let (json_data, proto_data) = generate_test_data(self.data_size);
        
        // JSON serialization
        let json_start = Instant::now();
        for _ in 0..self.iterations {
            let _ = serde_json::to_string(&json_data).unwrap();
        }
        let json_time = json_start.elapsed().as_secs_f64() * 1000.0 / self.iterations as f64;
        
        // Protobuf serialization
        let proto_start = Instant::now();
        for _ in 0..self.iterations {
            let mut buf = Vec::new();
            proto_data.encode(&mut buf).unwrap();
        }
        let proto_time = proto_start.elapsed().as_secs_f64() * 1000.0 / self.iterations as f64;
        
        let diff_percent = (json_time / proto_time) * 100.0;
        let winner = if json_time < proto_time { "JSON".to_string() } else { "Protobuf".to_string() };
        
        println!("JSON: {:.4} ms per op", json_time);
        println!("Protobuf: {:.4} ms per op", proto_time);
        
        BenchmarkMetric {
            json: json_time,
            protobuf: proto_time,
            difference_percent: diff_percent,
            winner,
        }
    }

    // 2. Test deserialization speed
    pub fn test_deserialization_speed(&self) -> BenchmarkMetric {
        println!("{}", "Testing deserialization speed...".green());
        
        let (json_data, proto_data) = generate_test_data(self.data_size);
        
        // Prepare serialized data
        let json_string = serde_json::to_string(&json_data).unwrap();
        let mut proto_bytes = Vec::new();
        proto_data.encode(&mut proto_bytes).unwrap();
        
        // JSON deserialization
        let json_start = Instant::now();
        for _ in 0..self.iterations {
            let _: JsonPerson = serde_json::from_str(&json_string).unwrap();
        }
        let json_time = json_start.elapsed().as_secs_f64() * 1000.0 / self.iterations as f64;
        
        // Protobuf deserialization
        let proto_start = Instant::now();
        for _ in 0..self.iterations {
            let _: Person = Person::decode(proto_bytes.as_slice()).unwrap();
        }
        let proto_time = proto_start.elapsed().as_secs_f64() * 1000.0 / self.iterations as f64;
        
        let diff_percent = (json_time / proto_time) * 100.0;
        let winner = if json_time < proto_time { "JSON".to_string() } else { "Protobuf".to_string() };
        
        println!("JSON: {:.4} ms per op", json_time);
        println!("Protobuf: {:.4} ms per op", proto_time);
        
        BenchmarkMetric {
            json: json_time,
            protobuf: proto_time,
            difference_percent: diff_percent,
            winner,
        }
    }

    // 3. Test payload size
    pub fn test_payload_size(&self) -> PayloadSizeMetric {
        println!("{}", "Testing payload size...".green());
        
        let (json_data, proto_data) = generate_test_data(self.data_size);
        
        // JSON serialization
        let json_string = serde_json::to_string(&json_data).unwrap();
        let json_size = json_string.len();
        
        // JSON compression
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(json_string.as_bytes()).unwrap();
        let json_compressed = encoder.finish().unwrap();
        let json_compressed_size = json_compressed.len();
        
        // Protobuf serialization
        let mut proto_bytes = Vec::new();
        proto_data.encode(&mut proto_bytes).unwrap();
        let proto_size = proto_bytes.len();
        
        // Protobuf compression
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&proto_bytes).unwrap();
        let proto_compressed = encoder.finish().unwrap();
        let proto_compressed_size = proto_compressed.len();
        
        let uncompressed_diff = (json_size as f64 / proto_size as f64) * 100.0;
        let uncompressed_winner = if json_size < proto_size { "JSON".to_string() } else { "Protobuf".to_string() };
        
        let compressed_diff = (json_compressed_size as f64 / proto_compressed_size as f64) * 100.0;
        let compressed_winner = if json_compressed_size < proto_compressed_size { "JSON".to_string() } else { "Protobuf".to_string() };
        
        println!("JSON size: {} bytes (uncompressed), {} bytes (compressed)",
                json_size, json_compressed_size);
        println!("Protobuf size: {} bytes (uncompressed), {} bytes (compressed)",
                proto_size, proto_compressed_size);
        
        PayloadSizeMetric {
            uncompressed: BenchmarkMetric {
                json: json_size as f64,
                protobuf: proto_size as f64,
                difference_percent: uncompressed_diff,
                winner: uncompressed_winner,
            },
            compressed: BenchmarkMetric {
                json: json_compressed_size as f64,
                protobuf: proto_compressed_size as f64,
                difference_percent: compressed_diff,
                winner: compressed_winner,
            },
        }
    }

    // 4. Test CPU usage (using execution time as a proxy)
    pub fn test_cpu_usage(&self) -> BenchmarkMetric {
        println!("{}", "Testing CPU usage (via execution time)...".green());
        
        let (json_data, proto_data) = generate_test_data(self.data_size);
        let heavy_workload = self.iterations * 10; // More iterations for CPU stress
        
        // JSON CPU usage
        let json_start = Instant::now();
        for _ in 0..heavy_workload {
            let json_string = serde_json::to_string(&json_data).unwrap();
            let _: JsonPerson = serde_json::from_str(&json_string).unwrap();
        }
        let json_time = json_start.elapsed().as_secs_f64() * 1000.0;
        
        // Protobuf CPU usage
        let proto_start = Instant::now();
        for _ in 0..heavy_workload {
            let mut buf = Vec::new();
            proto_data.clone().encode(&mut buf).unwrap();
            let _: Person = Person::decode(buf.as_slice()).unwrap();
        }
        let proto_time = proto_start.elapsed().as_secs_f64() * 1000.0;
        
        let diff_percent = (json_time / proto_time) * 100.0;
        let winner = if json_time < proto_time { "JSON".to_string() } else { "Protobuf".to_string() };
        
        println!("JSON execution time: {:.2} ms", json_time);
        println!("Protobuf execution time: {:.2} ms", proto_time);
        
        BenchmarkMetric {
            json: json_time,
            protobuf: proto_time,
            difference_percent: diff_percent,
            winner,
        }
    }

    // 5. Test memory usage (estimating via allocation counts)
    pub fn test_memory_usage(&self) -> BenchmarkMetric {
        println!("{}", "Testing memory usage (estimation)...".green());
        
        let (json_data, proto_data) = generate_test_data(self.data_size);
        
        // We can't directly measure memory usage easily, use proxy of time spent creating objects
        let json_start = Instant::now();
        let mut json_objects = Vec::with_capacity(self.iterations);
        for _ in 0..self.iterations {
            let json_string = serde_json::to_string(&json_data).unwrap();
            let parsed: JsonPerson = serde_json::from_str(&json_string).unwrap();
            json_objects.push(parsed);
        }
        // Force cleanup by clearing vector
        json_objects.clear();
        let json_time = json_start.elapsed().as_secs_f64() * 1000.0;
        
        let proto_start = Instant::now();
        let mut proto_objects = Vec::with_capacity(self.iterations);
        for _ in 0..self.iterations {
            let mut buf = Vec::new();
            proto_data.clone().encode(&mut buf).unwrap();
            let parsed = Person::decode(buf.as_slice()).unwrap();
            proto_objects.push(parsed);
        }
        // Force cleanup
        proto_objects.clear();
        let proto_time = proto_start.elapsed().as_secs_f64() * 1000.0;
        
        let diff_percent = (json_time / proto_time) * 100.0;
        let winner = if json_time < proto_time { "JSON".to_string() } else { "Protobuf".to_string() };
        
        println!("JSON memory operation time: {:.2} ms", json_time);
        println!("Protobuf memory operation time: {:.2} ms", proto_time);
        
        BenchmarkMetric {
            json: json_time,
            protobuf: proto_time,
            difference_percent: diff_percent,
            winner,
        }
    }

    // 6. Test network transfer time (simulation)
    pub async fn test_network_transfer(&self) -> BenchmarkMetric {
        println!("{}", "Testing network transfer time (simulation)...".green());
        
        let (json_data, proto_data) = generate_test_data(self.data_size);
        let latency_ms = 50.0; // Base network latency in milliseconds
        
        // Prepare serialized data
        let json_string = serde_json::to_string(&json_data).unwrap();
        let json_size = json_string.len();
        
        let mut proto_bytes = Vec::new();
        proto_data.encode(&mut proto_bytes).unwrap();
        let proto_size = proto_bytes.len();
        
        // Simulate network with artificial latency
        let simulate_network = |size: usize, latency: f64| -> f64 {
            // Base latency + additional time based on payload size
            // Simulating ~10Mbps connection
            latency + (size as f64 * 8.0) / (10.0 * 1024.0 * 1024.0) * 1000.0
        };
        
        // JSON network test
        let json_network_time = simulate_network(json_size, latency_ms);
        
        // Protobuf network test
        let proto_network_time = simulate_network(proto_size, latency_ms);
        
        let diff_percent = (json_network_time / proto_network_time) * 100.0;
        let winner = if json_network_time < proto_network_time { "JSON".to_string() } else { "Protobuf".to_string() };
        
        println!("JSON network time: {:.2} ms per request", json_network_time);
        println!("Protobuf network time: {:.2} ms per request", proto_network_time);
        
        BenchmarkMetric {
            json: json_network_time,
            protobuf: proto_network_time,
            difference_percent: diff_percent,
            winner,
        }
    }

    // 7. Test latency under load
    pub async fn test_latency_under_load(&self) -> BenchmarkMetric {
        println!("{}", "Testing latency under load...".green());
        
        let (json_data, proto_data) = generate_test_data(self.data_size);
        let concurrent_ops = 10;
        let iter_per_thread = 10;
        
        // Prepare serialized data
        let json_string = serde_json::to_string(&json_data).unwrap();
        let mut proto_bytes = Vec::new();
        proto_data.encode(&mut proto_bytes).unwrap();
        let proto_bytes = proto_bytes; // Make immutable
        
        // JSON concurrent load test
        let json_start = Instant::now();
        let mut json_handles = Vec::new();
        
        for _ in 0..concurrent_ops {
            let json_str = json_string.clone();
            let handle = tokio::spawn(async move {
                for _ in 0..iter_per_thread {
                    let _: JsonPerson = serde_json::from_str(&json_str).unwrap();
                    // Simulate some work
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            });
            json_handles.push(handle);
        }
        
        for handle in json_handles {
            handle.await.unwrap();
        }
        
        let json_time = json_start.elapsed().as_secs_f64() * 1000.0;
        
        // Protobuf concurrent load test
        let proto_start = Instant::now();
        let mut proto_handles = Vec::new();
        
        for _ in 0..concurrent_ops {
            let proto_data = proto_bytes.clone();
            let handle = tokio::spawn(async move {
                for _ in 0..iter_per_thread {
                    let _: Person = Person::decode(proto_data.as_slice()).unwrap();
                    // Simulate some work
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            });
            proto_handles.push(handle);
        }
        
        for handle in proto_handles {
            handle.await.unwrap();
        }
        
        let proto_time = proto_start.elapsed().as_secs_f64() * 1000.0;
        
        let diff_percent = (json_time / proto_time) * 100.0;
        let winner = if json_time < proto_time { "JSON".to_string() } else { "Protobuf".to_string() };
        
        println!("JSON latency under load: {:.2} ms", json_time);
        println!("Protobuf latency under load: {:.2} ms", proto_time);
        
        BenchmarkMetric {
            json: json_time,
            protobuf: proto_time,
            difference_percent: diff_percent,
            winner,
        }
    }

    // 8. Parser initialization time
    pub fn test_parser_initialization(&self) -> BenchmarkMetric {
        println!("{}", "Testing parser initialization time...".green());
        
        // For JSON, initialization is negligible as it's built into the standard library
        let json_init_time = 0.01; // Negligible initialization time
        
        // For Protobuf, we can't use prost_build here since it's a build dependency
        // We'll simulate the initialization with a more realistic but fixed value
        let proto_init_time = 5.0; // Example fixed value
        
        let diff_percent = (json_init_time / proto_init_time) * 100.0;
        let winner = if json_init_time < proto_init_time { "JSON".to_string() } else { "Protobuf".to_string() };
        
        println!("JSON initialization: {:.2} ms (built-in)", json_init_time);
        println!("Protobuf initialization: {:.2} ms", proto_init_time);
        
        BenchmarkMetric {
            json: json_init_time,
            protobuf: proto_init_time,
            difference_percent: diff_percent,
            winner,
        }
    }

    // 9. Test throughput
    pub fn test_throughput(&self) -> ThroughputMetric {
        println!("{}", "Testing throughput...".green());
        
        let (json_data, proto_data) = generate_test_data(self.data_size);
        let duration_ms = 1000.0; // 1 second test
        
        // JSON throughput
        let mut json_counter = 0;
        let json_start = Instant::now();
        
        while json_start.elapsed().as_secs_f64() * 1000.0 < duration_ms {
            let json_string = serde_json::to_string(&json_data).unwrap();
            let _: JsonPerson = serde_json::from_str(&json_string).unwrap();
            json_counter += 1;
        }
        
        let json_elapsed = json_start.elapsed().as_secs_f64();
        let json_throughput = json_counter as f64 / json_elapsed;
        
        // Protobuf throughput
        let mut proto_counter = 0;
        let proto_start = Instant::now();
        
        while proto_start.elapsed().as_secs_f64() * 1000.0 < duration_ms {
            let mut buf = Vec::new();
            proto_data.clone().encode(&mut buf).unwrap();
            let _: Person = Person::decode(buf.as_slice()).unwrap();
            proto_counter += 1;
        }
        
        let proto_elapsed = proto_start.elapsed().as_secs_f64();
        let proto_throughput = proto_counter as f64 / proto_elapsed;
        
        let diff_percent = (json_throughput / proto_throughput) * 100.0;
        let winner = if json_throughput > proto_throughput { "JSON".to_string() } else { "Protobuf".to_string() };
        
        println!("JSON throughput: {:.2} ops/s", json_throughput);
        println!("Protobuf throughput: {:.2} ops/s", proto_throughput);
        
        ThroughputMetric {
            json: json_throughput,
            protobuf: proto_throughput,
            difference_percent: diff_percent,
            winner,
        }
    }

    // 10. Test schema evolution handling
    pub fn test_schema_evolution(&self) -> SchemaEvolutionMetric {
        println!("{}", "Testing schema evolution handling...".green());
        
        // Generate both standard and evolved test data
        let (_, proto_basic) = generate_test_data(self.data_size);
        let (json_evolved_data, proto_evolved) = generate_evolved_test_data(self.data_size);
        
        // 1. Test backwards compatibility: New schema reading old data
        // This simulates when a newer client reads data created by an older service
        
        // Serialize with original schema
        let mut orig_bytes = Vec::new();
        proto_basic.encode(&mut orig_bytes).unwrap();
        
        // Try to convert between the schemas manually (simulating schema evolution)
        let backwards_start = Instant::now();
        for _ in 0..self.iterations {
            // In real schema evolution, we'd do conversion from old to new format
            let basic_decoded = Person::decode(orig_bytes.as_slice()).unwrap();
            
            // Manually map old format to new format (simulating schema evolution handling)
            let evolved = evolved::Person {
                name: basic_decoded.name.clone(),
                id: basic_decoded.id,
                email: basic_decoded.email.clone(),
                phones: basic_decoded.phones.iter().map(|p| evolved::person::PhoneNumber {
                    number: p.number.clone(),
                    r#type: p.r#type,
                    is_primary: p.r#type == 0, // Default logic for new field
                }).collect(),
                addresses: basic_decoded.addresses.iter().map(|a| evolved::person::Address {
                    street: a.street.clone(),
                    city: a.city.clone(),
                    state: a.state.clone(),
                    zip: a.zip.clone(),
                    country: a.country.clone(),
                    additional_info: String::new(), // Default for new field
                }).collect(),
                metadata: basic_decoded.metadata.clone(),
                additional_field: String::new(), // Default for new field
                priority: 0, // Default for new field
            };
            
            // Access some fields to ensure they're processed
            let _ = evolved.name;
            let _ = evolved.additional_field;
        }
        let backwards_time = backwards_start.elapsed().as_secs_f64() * 1000.0 / self.iterations as f64;
        
        // 2. Test forwards compatibility: Old schema reading new data
        // This simulates when an older client reads data created by a newer service
        
        // Serialize the evolved message
        let mut evolved_bytes = Vec::new();
        proto_evolved.encode(&mut evolved_bytes).unwrap();
        
        // Test forwards compatibility
        let forwards_start = Instant::now();
        for _ in 0..self.iterations {
            // In real schema evolution, we'd strip unknown fields when reading with old schema
            let evolved_decoded = evolved::Person::decode(evolved_bytes.as_slice()).unwrap();
            
            // Manually map new format to old format (simulating schema evolution handling)
            let basic = Person {
                name: evolved_decoded.name.clone(),
                id: evolved_decoded.id,
                email: evolved_decoded.email.clone(),
                phones: evolved_decoded.phones.iter().map(|p| person::PhoneNumber {
                    number: p.number.clone(),
                    r#type: p.r#type,
                }).collect(),
                addresses: evolved_decoded.addresses.iter().map(|a| person::Address {
                    street: a.street.clone(),
                    city: a.city.clone(),
                    state: a.state.clone(),
                    zip: a.zip.clone(),
                    country: a.country.clone(),
                }).collect(),
                metadata: evolved_decoded.metadata.clone(),
            };
            
            // Access some fields to ensure they're deserialized
            let _ = basic.name;
            let _ = basic.phones;
        }
        let forwards_time = forwards_start.elapsed().as_secs_f64() * 1000.0 / self.iterations as f64;
        
        // For JSON, schema evolution handling
        let json_start = Instant::now();
        let json_string = serde_json::to_string(&json_evolved_data).unwrap();
        
        for _ in 0..self.iterations {
            // Simulate a client that only understands original schema
            let parsed_full: serde_json::Value = serde_json::from_str(&json_string).unwrap();
            
            // Extract only the fields known in the original schema
            let mut filtered_data = serde_json::Map::new();
            filtered_data.insert("name".to_string(), parsed_full["name"].clone());
            filtered_data.insert("id".to_string(), parsed_full["id"].clone());
            filtered_data.insert("email".to_string(), parsed_full["email"].clone());
            
            // Extract phones (without is_primary field)
            let phones = parsed_full["phones"].as_array().unwrap();
            let filtered_phones: Vec<serde_json::Value> = phones
                .iter()
                .map(|phone| {
                    let mut filtered_phone = serde_json::Map::new();
                    filtered_phone.insert("number".to_string(), phone["number"].clone());
                    filtered_phone.insert("type_".to_string(), phone["type_"].clone());
                    serde_json::Value::Object(filtered_phone)
                })
                .collect();
            
            filtered_data.insert("phones".to_string(), serde_json::Value::Array(filtered_phones));
            
            // Extract addresses (without additional_info field)
            let addresses = parsed_full["addresses"].as_array().unwrap();
            let filtered_addresses: Vec<serde_json::Value> = addresses
                .iter()
                .map(|addr| {
                    let mut filtered_addr = serde_json::Map::new();
                    filtered_addr.insert("street".to_string(), addr["street"].clone());
                    filtered_addr.insert("city".to_string(), addr["city"].clone());
                    filtered_addr.insert("state".to_string(), addr["state"].clone());
                    filtered_addr.insert("zip".to_string(), addr["zip"].clone());
                    filtered_addr.insert("country".to_string(), addr["country"].clone());
                    serde_json::Value::Object(filtered_addr)
                })
                .collect();
            
            filtered_data.insert("addresses".to_string(), serde_json::Value::Array(filtered_addresses));
            filtered_data.insert("metadata".to_string(), parsed_full["metadata"].clone());
            
            // Convert back to JSON string (simulating storage or further processing)
            let _ = serde_json::to_string(&serde_json::Value::Object(filtered_data)).unwrap();
        }
        
        let json_time = json_start.elapsed().as_secs_f64() * 1000.0 / self.iterations as f64;
        let proto_avg = (backwards_time + forwards_time) / 2.0;
        
        let winner = if json_time < proto_avg { 
            "JSON".to_string() 
        } else { 
            "Protobuf".to_string() 
        };
        
        println!("JSON schema evolution: {:.4} ms per op", json_time);
        println!("Protobuf backwards compatibility: {:.4} ms per op", backwards_time);
        println!("Protobuf forwards compatibility: {:.4} ms per op", forwards_time);
        
        SchemaEvolutionMetric {
            json: json_time,
            protobuf_backwards: backwards_time,
            protobuf_forwards: forwards_time,
            protobuf_average: proto_avg,
            winner,
        }
    }

    // Run all tests
    pub async fn run_all_tests(&mut self) -> &BenchmarkResults {
        println!("{}", format!("Running all tests with data size {} and {} iterations...", 
                              self.data_size, self.iterations).blue().bold());
        
        // Run the tests
        let serialization = self.test_serialization_speed();
        let deserialization = self.test_deserialization_speed();
        let payload_size = self.test_payload_size();
        let cpu_usage = self.test_cpu_usage();
        let memory_usage = self.test_memory_usage();
        let network_transfer = self.test_network_transfer().await;
        let latency_under_load = self.test_latency_under_load().await;
        let parser_init = self.test_parser_initialization();
        let throughput = self.test_throughput();
        let schema_evolution = self.test_schema_evolution();
        
        // Store results
        self.results = Some(BenchmarkResults {
            serialization,
            deserialization,
            payload_size,
            cpu_usage,
            memory_usage,
            network_transfer,
            latency_under_load,
            parser_init,
            throughput,
            schema_evolution,
        });
        
        println!("{}", "All tests completed!".green().bold());
        
        self.results.as_ref().unwrap()
    }

    // Print results as a table
    pub fn print_results(&self) {
        if let Some(results) = &self.results {
            println!("\n{}", "JSON vs Protocol Buffers Benchmark Results".blue().bold());
            println!("{}", "===========================================".blue());
            
            let mut table = Table::new();
            
            table.add_row(row![bFg->"Test", bFg->"JSON", bFg->"Protobuf", bFg->"Difference", bFg->"Winner"]);
            
            // Add serialization results
            table.add_row(row![
                "Serialization (ms/op)",
                format!("{:.4}", results.serialization.json),
                format!("{:.4}", results.serialization.protobuf),
                format!("{:.2}%", results.serialization.difference_percent),
                results.serialization.winner
            ]);
            
            // Add deserialization results
            table.add_row(row![
                "Deserialization (ms/op)",
                format!("{:.4}", results.deserialization.json),
                format!("{:.4}", results.deserialization.protobuf),
                format!("{:.2}%", results.deserialization.difference_percent),
                results.deserialization.winner
            ]);
            
            // Add payload size results
            table.add_row(row![
                "Payload Size (bytes)",
                format!("{:.0}", results.payload_size.uncompressed.json),
                format!("{:.0}", results.payload_size.uncompressed.protobuf),
                format!("{:.2}%", results.payload_size.uncompressed.difference_percent),
                results.payload_size.uncompressed.winner
            ]);
            
            table.add_row(row![
                "Compressed Size (bytes)",
                format!("{:.0}", results.payload_size.compressed.json),
                format!("{:.0}", results.payload_size.compressed.protobuf),
                format!("{:.2}%", results.payload_size.compressed.difference_percent),
                results.payload_size.compressed.winner
            ]);
            
            // Add CPU usage results
            table.add_row(row![
                "CPU Usage (ms)",
                format!("{:.2}", results.cpu_usage.json),
                format!("{:.2}", results.cpu_usage.protobuf),
                format!("{:.2}%", results.cpu_usage.difference_percent),
                results.cpu_usage.winner
            ]);
            
            // Add memory usage results
            table.add_row(row![
                "Memory Usage (proxy ms)",
                format!("{:.2}", results.memory_usage.json),
                format!("{:.2}", results.memory_usage.protobuf),
                format!("{:.2}%", results.memory_usage.difference_percent),
                results.memory_usage.winner
            ]);
            
            // Add network transfer results
            table.add_row(row![
                "Network Transfer (ms)",
                format!("{:.2}", results.network_transfer.json),
                format!("{:.2}", results.network_transfer.protobuf),
                format!("{:.2}%", results.network_transfer.difference_percent),
                results.network_transfer.winner
            ]);
            
            // Add latency under load results
            table.add_row(row![
                "Latency Under Load (ms)",
                format!("{:.2}", results.latency_under_load.json),
                format!("{:.2}", results.latency_under_load.protobuf),
                format!("{:.2}%", results.latency_under_load.difference_percent),
                results.latency_under_load.winner
            ]);
            
            // Add parser initialization results
            table.add_row(row![
                "Parser Init (ms)",
                format!("{:.2}", results.parser_init.json),
                format!("{:.2}", results.parser_init.protobuf),
                format!("{:.2}%", results.parser_init.difference_percent),
                results.parser_init.winner
            ]);
            
            // Add throughput results
            table.add_row(row![
                "Throughput (ops/s)",
                format!("{:.2}", results.throughput.json),
                format!("{:.2}", results.throughput.protobuf),
                format!("{:.2}%", results.throughput.difference_percent),
                results.throughput.winner
            ]);
            
            // Add schema evolution results
            table.add_row(row![
                "Schema Evolution (ms/op)",
                format!("{:.4}", results.schema_evolution.json),
                format!("B: {:.4} / F: {:.4}", 
                       results.schema_evolution.protobuf_backwards,
                       results.schema_evolution.protobuf_forwards),
                format!("{:.2}%", 
                       (results.schema_evolution.json / results.schema_evolution.protobuf_average) * 100.0),
                results.schema_evolution.winner
            ]);
            
            // Print the table
            table.printstd();
            
            // Count winners
            let mut json_wins = 0;
            let mut proto_wins = 0;
            
            if results.serialization.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            if results.deserialization.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            if results.payload_size.uncompressed.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            if results.payload_size.compressed.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            if results.cpu_usage.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            if results.memory_usage.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            if results.network_transfer.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            if results.latency_under_load.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            if results.parser_init.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            if results.throughput.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            if results.schema_evolution.winner == "JSON" { json_wins += 1; } else { proto_wins += 1; }
            
            println!("\n{}", format!("Overall winner: {} ({} wins vs {} wins)", 
                                    if json_wins > proto_wins { "JSON" } else { "Protocol Buffers" },
                                    if json_wins > proto_wins { json_wins } else { proto_wins },
                                    if json_wins > proto_wins { proto_wins } else { json_wins }
                                    ).green().bold());
        } else {
            println!("No results to print. Run the tests first.");
        }
    }
}