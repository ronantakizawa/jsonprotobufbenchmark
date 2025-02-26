#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <map>
#include <iomanip>
#include <memory>
#include <algorithm>

// Structure to store benchmark results
struct BenchmarkResult {
    int data_size;
    
    // Serialization speed
    double json_ser_time;
    double protobuf_ser_time;
    double ser_ratio;  // JSON time / Protobuf time
    
    // Deserialization speed
    double json_deser_time;
    double protobuf_deser_time;
    double deser_ratio;  // JSON time / Protobuf time
    
    // Payload size
    size_t json_size;
    size_t protobuf_size;
    double size_ratio;  // JSON size / Protobuf size
    
    // Compressed sizes
    size_t json_compressed;
    size_t protobuf_compressed;
    double json_compression_ratio;     // compressed / original
    double protobuf_compression_ratio; // compressed / original
    
    // CPU usage
    double json_cpu;
    double protobuf_cpu;
    double cpu_ratio;  // JSON CPU / Protobuf CPU
    
    // Memory usage
    double json_memory;
    double protobuf_memory;
    double memory_ratio;  // JSON memory / Protobuf memory
    
    // Throughput
    double json_throughput;
    double protobuf_throughput;
    double throughput_ratio;  // Protobuf throughput / JSON throughput
};

// Function to analyze benchmark results
void analyzeResults(const std::vector<BenchmarkResult>& results) {
    std::cout << "====== JSON vs Protocol Buffers Performance Analysis ======" << std::endl;
    
    // Calculate averages across different data sizes
    double avg_ser_ratio = 0;
    double avg_deser_ratio = 0;
    double avg_size_ratio = 0;
    double avg_cpu_ratio = 0;
    double avg_memory_ratio = 0;
    double avg_throughput_ratio = 0;
    
    for (const auto& result : results) {
        avg_ser_ratio += result.ser_ratio;
        avg_deser_ratio += result.deser_ratio;
        avg_size_ratio += result.size_ratio;
        avg_cpu_ratio += result.cpu_ratio;
        avg_memory_ratio += result.memory_ratio;
        avg_throughput_ratio += result.throughput_ratio;
    }
    
    int num_results = results.size();
    avg_ser_ratio /= num_results;
    avg_deser_ratio /= num_results;
    avg_size_ratio /= num_results;
    avg_cpu_ratio /= num_results;
    avg_memory_ratio /= num_results;
    avg_throughput_ratio /= num_results;
    
    // Print summary results
    std::cout << "Average performance across all data sizes:" << std::endl;
    std::cout << "1. Serialization: Protocol Buffers is " << std::fixed << std::setprecision(2) 
              << avg_ser_ratio << "x faster than JSON" << std::endl;
    std::cout << "2. Deserialization: Protocol Buffers is " << std::fixed << std::setprecision(2) 
              << avg_deser_ratio << "x faster than JSON" << std::endl;
    std::cout << "3. Payload Size: Protocol Buffers is " << std::fixed << std::setprecision(2) 
              << avg_size_ratio << "x smaller than JSON" << std::endl;
    std::cout << "4. CPU Usage: Protocol Buffers uses " << std::fixed << std::setprecision(2) 
              << avg_cpu_ratio << "x less CPU than JSON" << std::endl;
    std::cout << "5. Memory Usage: Protocol Buffers uses " << std::fixed << std::setprecision(2) 
              << avg_memory_ratio << "x less memory than JSON" << std::endl;
    std::cout << "6. Throughput: Protocol Buffers processes " << std::fixed << std::setprecision(2) 
              << avg_throughput_ratio << "x more messages per second than JSON" << std::endl;
    
    // Analyze scaling with data size
    std::cout << "\nScaling with data size (performance ratio change from smallest to largest dataset):" << std::endl;
    if (num_results >= 2) {
        double scaling_ser = results.back().ser_ratio / results.front().ser_ratio;
        double scaling_deser = results.back().deser_ratio / results.front().deser_ratio;
        double scaling_size = results.back().size_ratio / results.front().size_ratio;
        
        std::cout << "- Serialization performance scaling: " << std::fixed << std::setprecision(2) << scaling_ser << "x" << std::endl;
        std::cout << "- Deserialization performance scaling: " << std::fixed << std::setprecision(2) << scaling_deser << "x" << std::endl;
        std::cout << "- Size advantage scaling: " << std::fixed << std::setprecision(2) << scaling_size << "x" << std::endl;
        
        if (scaling_ser > 1.0) 
            std::cout << "  ➜ Protocol Buffers' serialization advantage increases with data size" << std::endl;
        else if (scaling_ser < 1.0)
            std::cout << "  ➜ Protocol Buffers' serialization advantage decreases with data size" << std::endl;
            
        if (scaling_deser > 1.0) 
            std::cout << "  ➜ Protocol Buffers' deserialization advantage increases with data size" << std::endl;
        else if (scaling_deser < 1.0)
            std::cout << "  ➜ Protocol Buffers' deserialization advantage decreases with data size" << std::endl;
            
        if (scaling_size > 1.0) 
            std::cout << "  ➜ Protocol Buffers' size advantage increases with data size" << std::endl;
        else if (scaling_size < 1.0)
            std::cout << "  ➜ Protocol Buffers' size advantage decreases with data size" << std::endl;
    }
    
    // Compression analysis
    std::cout << "\nCompression efficiency:" << std::endl;
    double avg_json_compression = 0;
    double avg_protobuf_compression = 0;
    
    for (const auto& result : results) {
        avg_json_compression += result.json_compression_ratio;
        avg_protobuf_compression += result.protobuf_compression_ratio;
    }
    
    avg_json_compression /= num_results;
    avg_protobuf_compression /= num_results;
    
    std::cout << "- JSON compresses to " << std::fixed << std::setprecision(1) 
              << (avg_json_compression * 100) << "% of original size" << std::endl;
    std::cout << "- Protocol Buffers compresses to " << std::fixed << std::setprecision(1) 
              << (avg_protobuf_compression * 100) << "% of original size" << std::endl;
    
    if (avg_json_compression < avg_protobuf_compression)
        std::cout << "  ➜ JSON achieves better compression ratio (already being more verbose)" << std::endl;
    else
        std::cout << "  ➜ Protocol Buffers achieves better compression ratio (despite being more compact to begin with)" << std::endl;
    
    // Performance recommendations
    std::cout << "\n====== Performance Recommendations ======" << std::endl;
    
    std::cout << "Based on the benchmark results, here are recommendations for different use cases:" << std::endl;
    
    std::cout << "\n1. Use Protocol Buffers when:" << std::endl;
    std::cout << "   ✓ Performance is critical (especially for high-throughput systems)" << std::endl;
    std::cout << "   ✓ Network bandwidth is constrained" << std::endl;
    std::cout << "   ✓ Processing large amounts of data" << std::endl;
    std::cout << "   ✓ Implementing RPC systems" << std::endl;
    std::cout << "   ✓ Backward/forward compatibility is important" << std::endl;
    std::cout << "   ✓ Multiple language support is needed with consistent schema" << std::endl;
    
    std::cout << "\n2. Use JSON when:" << std::endl;
    std::cout << "   ✓ Human readability is required" << std::endl;
    std::cout << "   ✓ Rapid development without schema definition is needed" << std::endl;
    std::cout << "   ✓ Working directly with web browsers" << std::endl;
    std::cout << "   ✓ Flexibility and schema-less operation is preferred" << std::endl;
    std::cout << "   ✓ Debugging and manual data inspection is important" << std::endl;
    std::cout << "   ✓ Performance is not the primary concern" << std::endl;
    
    std::cout << "\n3. Hybrid approach:" << std::endl;
    std::cout << "   ✓ Use Protocol Buffers for internal system communication" << std::endl;
    std::cout << "   ✓ Use JSON for external APIs and user-facing interfaces" << std::endl;
    std::cout << "   ✓ Implement converters between Protocol Buffers and JSON" << std::endl;
}

// Example usage (in a real implementation, this would process actual benchmark results)
int main() {
    // Sample results based on typical Protocol Buffers vs JSON performance 
    std::vector<BenchmarkResult> results = {
        // Small data set (10 items)
        {
            .data_size = 10,
            .json_ser_time = 0.0050,
            .protobuf_ser_time = 0.0030,
            .ser_ratio = 1.67,
            .json_deser_time = 0.0070,
            .protobuf_deser_time = 0.0025,
            .deser_ratio = 2.80,
            .json_size = 650,
            .protobuf_size = 320,
            .size_ratio = 2.03,
            .json_compressed = 220,
            .protobuf_compressed = 160,
            .json_compression_ratio = 0.34,
            .protobuf_compression_ratio = 0.50,
            .json_cpu = 65.0,
            .protobuf_cpu = 45.0,
            .cpu_ratio = 1.44,
            .json_memory = 1.8,
            .protobuf_memory = 1.2,
            .memory_ratio = 1.50,
            .json_throughput = 8500,
            .protobuf_throughput = 15000,
            .throughput_ratio = 1.76
        },
        
        // Medium data set (100 items)
        {
            .data_size = 100,
            .json_ser_time = 0.0450,
            .protobuf_ser_time = 0.0180,
            .ser_ratio = 2.50,
            .json_deser_time = 0.0580,
            .protobuf_deser_time = 0.0150,
            .deser_ratio = 3.87,
            .json_size = 5800,
            .protobuf_size = 2200,
            .size_ratio = 2.64,
            .json_compressed = 1450,
            .protobuf_compressed = 980,
            .json_compression_ratio = 0.25,
            .protobuf_compression_ratio = 0.45,
            .json_cpu = 72.0,
            .protobuf_cpu = 46.0,
            .cpu_ratio = 1.57,
            .json_memory = 5.2,
            .protobuf_memory = 2.8,
            .memory_ratio = 1.86,
            .json_throughput = 950,
            .protobuf_throughput = 2800,
            .throughput_ratio = 2.95
        },
        
        // Large data set (1000 items)
        {
            .data_size = 1000,
            .json_ser_time = 0.4200,
            .protobuf_ser_time = 0.1250,
            .ser_ratio = 3.36,
            .json_deser_time = 0.5100,
            .protobuf_deser_time = 0.0950,
            .deser_ratio = 5.37,
            .json_size = 58000,
            .protobuf_size = 20500,
            .size_ratio = 2.83,
            .json_compressed = 12000,
            .protobuf_compressed = 8000,
            .json_compression_ratio = 0.21,
            .protobuf_compression_ratio = 0.39,
            .json_cpu = 78.0,
            .protobuf_cpu = 47.0,
            .cpu_ratio = 1.66,
            .json_memory = 45.0,
            .protobuf_memory = 22.0,
            .memory_ratio = 2.05,
            .json_throughput = 105,
            .protobuf_throughput = 410,
            .throughput_ratio = 3.90
        }
    };
    
    analyzeResults(results);
    
    return 0;
}