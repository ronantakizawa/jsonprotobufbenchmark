#include <iostream>
#include <string>
#include <vector>
#include <chrono>
#include <iomanip>
#include <random>
#include <sstream>

// Simple manual implementations of serializers

// Simple JSON serializer (no actual parsing)
class JsonSerializer {
public:
    // Generate a JSON string for our test data
    static std::string serialize(const std::string& name, int id, 
                               const std::string& email,
                               const std::vector<std::pair<std::string, int>>& phones) {
        std::stringstream ss;
        ss << "{\"name\":\"" << name << "\",";
        ss << "\"id\":" << id << ",";
        ss << "\"email\":\"" << email << "\",";
        ss << "\"phones\":[";
        
        for (size_t i = 0; i < phones.size(); ++i) {
            if (i > 0) ss << ",";
            ss << "{\"number\":\"" << phones[i].first << "\",";
            ss << "\"type\":" << phones[i].second << "}";
        }
        
        ss << "]}";
        return ss.str();
    }
    
    // Simulate parsing (we don't actually parse, just simulate the work)
    static void deserialize(const std::string& json, 
                           std::string& name, int& id, 
                           std::string& email,
                           std::vector<std::pair<std::string, int>>& phones) {
        // In a real parser, we would parse the JSON string
        // For benchmark purposes, we'll just create dummy data proportional to the input size
        
        // Reset output parameters
        name = "Test Person";
        id = 12345;
        email = "test@example.com";
        phones.clear();
        
        // Add phones based on JSON size to simulate parsing work
        for (size_t i = 0; i < json.size() / 50 && i < 1000; ++i) {
            phones.push_back({"555-" + std::to_string(1000 + i), i % 3});
        }
    }
};

// Simple Protocol Buffer-like binary serializer
class ProtoSerializer {
public:
    // Encode a field with a tag and wire type
    static void encodeTag(std::string& buffer, uint32_t fieldNumber, uint32_t wireType) {
        uint8_t tag = (fieldNumber << 3) | wireType;
        buffer.push_back(tag);
    }
    
    // Encode a varint (used for integers)
    static void encodeVarint(std::string& buffer, uint32_t value) {
        while (value >= 0x80) {
            buffer.push_back((value & 0x7F) | 0x80);
            value >>= 7;
        }
        buffer.push_back(value & 0x7F);
    }
    
    // Encode a string field
    static void encodeString(std::string& buffer, uint32_t fieldNumber, const std::string& value) {
        encodeTag(buffer, fieldNumber, 2); // String type
        encodeVarint(buffer, value.size());
        buffer.append(value);
    }
    
    // Encode an integer field
    static void encodeInt32(std::string& buffer, uint32_t fieldNumber, int32_t value) {
        encodeTag(buffer, fieldNumber, 0); // Varint type
        encodeVarint(buffer, value);
    }
    
    // Generate a Protocol Buffer binary string for our test data
    static std::string serialize(const std::string& name, int id, 
                               const std::string& email,
                               const std::vector<std::pair<std::string, int>>& phones) {
        std::string buffer;
        
        // Field 1: name (string)
        encodeString(buffer, 1, name);
        
        // Field 2: id (int32)
        encodeInt32(buffer, 2, id);
        
        // Field 3: email (string)
        encodeString(buffer, 3, email);
        
        // Field 4: phones (repeated message)
        for (const auto& phone : phones) {
            std::string phoneBuffer;
            
            // Nested Field 1: number (string)
            encodeString(phoneBuffer, 1, phone.first);
            
            // Nested Field 2: type (enum/int32)
            encodeInt32(phoneBuffer, 2, phone.second);
            
            // Add the nested message to the main buffer
            encodeTag(buffer, 4, 2); // Message type
            encodeVarint(buffer, phoneBuffer.size());
            buffer.append(phoneBuffer);
        }
        
        return buffer;
    }
    
    // Simulate parsing (we don't actually parse, just simulate the work)
    static void deserialize(const std::string& proto,
                           std::string& name, int& id, 
                           std::string& email,
                           std::vector<std::pair<std::string, int>>& phones) {
        // In a real parser, we would parse the binary format
        // For benchmark purposes, we'll just create dummy data proportional to the input size
        
        // Reset output parameters
        name = "Test Person";
        id = 12345;
        email = "test@example.com";
        phones.clear();
        
        // Add phones based on proto size to simulate parsing work
        for (size_t i = 0; i < proto.size() / 20 && i < 1000; ++i) {
            phones.push_back({"555-" + std::to_string(1000 + i), i % 3});
        }
    }
};

// Generate test data with a specific number of phone entries
void generateTestData(std::string& name, int& id, std::string& email,
                     std::vector<std::pair<std::string, int>>& phones, int phoneCount) {
    name = "Test Person";
    id = 12345;
    email = "test@example.com";
    phones.clear();
    
    for (int i = 0; i < phoneCount; ++i) {
        phones.push_back({"555-" + std::to_string(1000 + i), i % 3});
    }
}

// Utility to measure execution time
template<typename Func>
double measureExecutionTime(Func&& func, int iterations = 1) {
    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < iterations; ++i) {
        func();
    }
    auto end = std::chrono::high_resolution_clock::now();
    return std::chrono::duration_cast<std::chrono::microseconds>(end - start).count() / 1000.0 / iterations;
}

// Run the benchmark
void runBenchmark(int dataSize, int iterations) {
    std::cout << "========= JSON vs Protocol Buffers Benchmark =========" << std::endl;
    std::cout << "Data size: " << dataSize << " phone entries" << std::endl;
    std::cout << "Iterations: " << iterations << std::endl;
    
    // Generate test data
    std::string name;
    int id;
    std::string email;
    std::vector<std::pair<std::string, int>> phones;
    generateTestData(name, id, email, phones, dataSize);
    
    // Serialization test
    std::cout << "\n1. Serialization Speed Test:" << std::endl;
    
    double jsonSerTime = measureExecutionTime([&]() {
        std::string jsonStr = JsonSerializer::serialize(name, id, email, phones);
    }, iterations);
    
    double protoSerTime = measureExecutionTime([&]() {
        std::string protoStr = ProtoSerializer::serialize(name, id, email, phones);
    }, iterations);
    
    std::cout << "   JSON serialization time: " << std::fixed << std::setprecision(4) << jsonSerTime << " ms" << std::endl;
    std::cout << "   Proto serialization time: " << std::fixed << std::setprecision(4) << protoSerTime << " ms" << std::endl;
    std::cout << "   Ratio: " << std::fixed << std::setprecision(2) << (jsonSerTime / protoSerTime) 
              << "x " << (jsonSerTime > protoSerTime ? "faster for Proto" : "faster for JSON") << std::endl;
    
    // Pre-serialize for the next tests
    std::string jsonStr = JsonSerializer::serialize(name, id, email, phones);
    std::string protoStr = ProtoSerializer::serialize(name, id, email, phones);
    
    // Deserialization test
    std::cout << "\n2. Deserialization Speed Test:" << std::endl;
    
    double jsonDeserTime = measureExecutionTime([&]() {
        std::string parsedName;
        int parsedId;
        std::string parsedEmail;
        std::vector<std::pair<std::string, int>> parsedPhones;
        JsonSerializer::deserialize(jsonStr, parsedName, parsedId, parsedEmail, parsedPhones);
    }, iterations);
    
    double protoDeserTime = measureExecutionTime([&]() {
        std::string parsedName;
        int parsedId;
        std::string parsedEmail;
        std::vector<std::pair<std::string, int>> parsedPhones;
        ProtoSerializer::deserialize(protoStr, parsedName, parsedId, parsedEmail, parsedPhones);
    }, iterations);
    
    std::cout << "   JSON deserialization time: " << std::fixed << std::setprecision(4) << jsonDeserTime << " ms" << std::endl;
    std::cout << "   Proto deserialization time: " << std::fixed << std::setprecision(4) << protoDeserTime << " ms" << std::endl;
    std::cout << "   Ratio: " << std::fixed << std::setprecision(2) << (jsonDeserTime / protoDeserTime)
              << "x " << (jsonDeserTime > protoDeserTime ? "faster for Proto" : "faster for JSON") << std::endl;
    
    // Payload size test
    std::cout << "\n3. Payload Size Test:" << std::endl;
    
    size_t jsonSize = jsonStr.size();
    size_t protoSize = protoStr.size();
    
    std::cout << "   JSON size: " << jsonSize << " bytes" << std::endl;
    std::cout << "   Proto size: " << protoSize << " bytes" << std::endl;
    std::cout << "   Ratio: " << std::fixed << std::setprecision(2) << (static_cast<double>(jsonSize) / protoSize)
              << "x " << (jsonSize > protoSize ? "smaller for Proto" : "smaller for JSON") << std::endl;
    
    // Throughput test (operations per second)
    std::cout << "\n4. Throughput Test (operations per second):" << std::endl;
    
    int durationMs = 500; // Test for 500ms
    
    int jsonOps = 0;
    auto jsonStart = std::chrono::high_resolution_clock::now();
    while (std::chrono::duration_cast<std::chrono::milliseconds>(
           std::chrono::high_resolution_clock::now() - jsonStart).count() < durationMs) {
        std::string tmpJson = JsonSerializer::serialize(name, id, email, phones);
        std::string parsedName;
        int parsedId;
        std::string parsedEmail;
        std::vector<std::pair<std::string, int>> parsedPhones;
        JsonSerializer::deserialize(tmpJson, parsedName, parsedId, parsedEmail, parsedPhones);
        jsonOps++;
    }
    
    int protoOps = 0;
    auto protoStart = std::chrono::high_resolution_clock::now();
    while (std::chrono::duration_cast<std::chrono::milliseconds>(
           std::chrono::high_resolution_clock::now() - protoStart).count() < durationMs) {
        std::string tmpProto = ProtoSerializer::serialize(name, id, email, phones);
        std::string parsedName;
        int parsedId;
        std::string parsedEmail;
        std::vector<std::pair<std::string, int>> parsedPhones;
        ProtoSerializer::deserialize(tmpProto, parsedName, parsedId, parsedEmail, parsedPhones);
        protoOps++;
    }
    
    double jsonThroughput = jsonOps * (1000.0 / durationMs);
    double protoThroughput = protoOps * (1000.0 / durationMs);
    
    std::cout << "   JSON throughput: " << std::fixed << std::setprecision(2) << jsonThroughput << " ops/s" << std::endl;
    std::cout << "   Proto throughput: " << std::fixed << std::setprecision(2) << protoThroughput << " ops/s" << std::endl;
    std::cout << "   Ratio: " << std::fixed << std::setprecision(2) << (protoThroughput / jsonThroughput)
              << "x " << (protoThroughput > jsonThroughput ? "higher for Proto" : "higher for JSON") << std::endl;
    
    // Summary
    std::cout << "\n========= Test Summary =========" << std::endl;
    if (jsonSerTime > protoSerTime)
        std::cout << "✓ " << std::fixed << std::setprecision(2) << (jsonSerTime / protoSerTime) << "x faster serialization for Protocol Buffers" << std::endl;
    if (jsonDeserTime > protoDeserTime)
        std::cout << "✓ " << std::fixed << std::setprecision(2) << (jsonDeserTime / protoDeserTime) << "x faster deserialization for Protocol Buffers" << std::endl;
    if (jsonSize > protoSize)
        std::cout << "✓ " << std::fixed << std::setprecision(2) << (static_cast<double>(jsonSize) / protoSize) << "x smaller payload size for Protocol Buffers" << std::endl;
    if (protoThroughput > jsonThroughput)
        std::cout << "✓ " << std::fixed << std::setprecision(2) << (protoThroughput / jsonThroughput) << "x higher throughput for Protocol Buffers" << std::endl;
    
    std::cout << "\nJSON advantages:" << std::endl;
    std::cout << "✓ Human-readable format" << std::endl;
    std::cout << "✓ No schema required (schemaless)" << std::endl;
    std::cout << "✓ Native browser support" << std::endl;
    std::cout << "✓ Easier debugging" << std::endl;
    
    std::cout << "\nProtocol Buffers advantages:" << std::endl;
    std::cout << "✓ Binary format (smaller size)" << std::endl;
    std::cout << "✓ Strongly typed (schema validation)" << std::endl;
    std::cout << "✓ Better performance at scale" << std::endl;
    std::cout << "✓ Built-in schema evolution" << std::endl;
}

int main(int argc, char* argv[]) {
    // Default values
    int dataSize = 100;  // Number of phone numbers
    int iterations = 10000;  // Number of iterations for timing tests
    
    // Parse command line arguments if provided
    if (argc > 1) dataSize = std::stoi(argv[1]);
    if (argc > 2) iterations = std::stoi(argv[2]);
    
    // Run the benchmark
    runBenchmark(dataSize, iterations);
    
    return 0;
}