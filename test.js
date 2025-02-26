// JSON vs Protocol Buffers Performance Test
import * as protobuf from 'protobufjs';
import pako from 'pako'; // For compression tests

// Define Protocol Buffer schema
const protoSchema = `
syntax = "proto3";
package test;

message Person {
  string name = 1;
  int32 id = 2;
  string email = 3;
  repeated PhoneNumber phones = 4;
  repeated Address addresses = 5;
  map<string, string> metadata = 6;

  message PhoneNumber {
    string number = 1;
    PhoneType type = 2;
  }

  enum PhoneType {
    MOBILE = 0;
    HOME = 1;
    WORK = 2;
  }
  
  message Address {
    string street = 1;
    string city = 2;
    string state = 3;
    string zip = 4;
    string country = 5;
  }
}
`;

// Generate test data with specified size
function generateTestData(size) {
  const result = {
    name: "Test Person",
    id: 12345,
    email: "test@example.com",
    phones: [],
    addresses: [],
    metadata: {}
  };
  
  // Add phone numbers based on size
  for (let i = 0; i < size; i++) {
    result.phones.push({
      number: `555-${1000 + i}`,
      type: i % 3 // Cycle through MOBILE, HOME, WORK
    });
  }
  
  // Add addresses based on size
  for (let i = 0; i < Math.max(1, size / 2); i++) {
    result.addresses.push({
      street: `${100 + i} Main St`,
      city: `City ${i}`,
      state: `State ${i}`,
      zip: `${10000 + i}`,
      country: "Country"
    });
  }
  
  // Add metadata based on size
  for (let i = 0; i < size; i++) {
    result.metadata[`key${i}`] = `value${i}`;
  }
  
  return result;
}

// Create a simple evolved schema (adding a new field)
const evolvedProtoSchema = `
syntax = "proto3";
package test;

message Person {
  string name = 1;
  int32 id = 2;
  string email = 3;
  repeated PhoneNumber phones = 4;
  repeated Address addresses = 5;
  map<string, string> metadata = 6;
  string additional_field = 7; // New field
  int32 priority = 8; // New field

  message PhoneNumber {
    string number = 1;
    PhoneType type = 2;
    bool is_primary = 3; // New field
  }

  enum PhoneType {
    MOBILE = 0;
    HOME = 1;
    WORK = 2;
    OTHER = 3; // New enum value
  }
  
  message Address {
    string street = 1;
    string city = 2;
    string state = 3;
    string zip = 4;
    string country = 5;
    string additional_info = 6; // New field
  }
}
`;

// Performance testing class
class PerformanceTester {
  constructor() {
    this.results = {};
  }
  
  async initialize() {
    console.log("Initializing Protocol Buffers...");
    const initStartTime = performance.now();
    
    // Load and parse the schema
    const root = protobuf.parse(protoSchema).root;
    this.PersonMessage = root.lookupType("test.Person");
    
    const evolvedRoot = protobuf.parse(evolvedProtoSchema).root;
    this.EvolvedPersonMessage = evolvedRoot.lookupType("test.Person");
    
    const initEndTime = performance.now();
    this.parserInitTime = initEndTime - initStartTime;
    
    console.log(`Protocol Buffers initialized in ${this.parserInitTime.toFixed(2)} ms`);
    return this;
  }
  
  // 1. Test serialization speed
  async testSerializationSpeed(data, iterations = 1000) {
    console.log("Testing serialization speed...");
    
    // JSON serialization
    const jsonStartTime = performance.now();
    for (let i = 0; i < iterations; i++) {
      JSON.stringify(data);
    }
    const jsonEndTime = performance.now();
    const jsonSerializationTime = (jsonEndTime - jsonStartTime) / iterations;
    
    // Protobuf serialization
    const message = this.PersonMessage.create(data);
    const protobufStartTime = performance.now();
    for (let i = 0; i < iterations; i++) {
      this.PersonMessage.encode(message).finish();
    }
    const protobufEndTime = performance.now();
    const protobufSerializationTime = (protobufEndTime - protobufStartTime) / iterations;
    
    this.results.serializationSpeed = {
      json: jsonSerializationTime,
      protobuf: protobufSerializationTime,
      difference: `${((jsonSerializationTime / protobufSerializationTime) * 100).toFixed(2)}%`,
      winner: jsonSerializationTime < protobufSerializationTime ? "JSON" : "Protobuf"
    };
    
    console.log(`JSON: ${jsonSerializationTime.toFixed(4)} ms per op`);
    console.log(`Protobuf: ${protobufSerializationTime.toFixed(4)} ms per op`);
    
    return this;
  }
  
  // 2. Test deserialization speed
  async testDeserializationSpeed(data, iterations = 1000) {
    console.log("Testing deserialization speed...");
    
    // Prepare serialized data
    const jsonData = JSON.stringify(data);
    const message = this.PersonMessage.create(data);
    const protobufData = this.PersonMessage.encode(message).finish();
    
    // JSON deserialization
    const jsonStartTime = performance.now();
    for (let i = 0; i < iterations; i++) {
      JSON.parse(jsonData);
    }
    const jsonEndTime = performance.now();
    const jsonDeserializationTime = (jsonEndTime - jsonStartTime) / iterations;
    
    // Protobuf deserialization
    const protobufStartTime = performance.now();
    for (let i = 0; i < iterations; i++) {
      this.PersonMessage.decode(protobufData);
    }
    const protobufEndTime = performance.now();
    const protobufDeserializationTime = (protobufEndTime - protobufStartTime) / iterations;
    
    this.results.deserializationSpeed = {
      json: jsonDeserializationTime,
      protobuf: protobufDeserializationTime,
      difference: `${((jsonDeserializationTime / protobufDeserializationTime) * 100).toFixed(2)}%`,
      winner: jsonDeserializationTime < protobufDeserializationTime ? "JSON" : "Protobuf"
    };
    
    console.log(`JSON: ${jsonDeserializationTime.toFixed(4)} ms per op`);
    console.log(`Protobuf: ${protobufDeserializationTime.toFixed(4)} ms per op`);
    
    return this;
  }
  
  // 3. Test payload size
  async testPayloadSize(data) {
    console.log("Testing payload size...");
    
    // JSON serialization
    const jsonData = JSON.stringify(data);
    const jsonSize = new TextEncoder().encode(jsonData).length;
    const jsonCompressed = pako.deflate(jsonData);
    const jsonCompressedSize = jsonCompressed.length;
    
    // Protobuf serialization
    const message = this.PersonMessage.create(data);
    const protobufData = this.PersonMessage.encode(message).finish();
    const protobufSize = protobufData.length;
    const protobufCompressed = pako.deflate(protobufData);
    const protobufCompressedSize = protobufCompressed.length;
    
    this.results.payloadSize = {
      uncompressed: {
        json: jsonSize,
        protobuf: protobufSize,
        difference: `${((jsonSize / protobufSize) * 100).toFixed(2)}%`,
        winner: jsonSize < protobufSize ? "JSON" : "Protobuf"
      },
      compressed: {
        json: jsonCompressedSize,
        protobuf: protobufCompressedSize,
        difference: `${((jsonCompressedSize / protobufCompressedSize) * 100).toFixed(2)}%`,
        winner: jsonCompressedSize < protobufCompressedSize ? "JSON" : "Protobuf"
      }
    };
    
    console.log(`JSON size: ${jsonSize} bytes (uncompressed), ${jsonCompressedSize} bytes (compressed)`);
    console.log(`Protobuf size: ${protobufSize} bytes (uncompressed), ${protobufCompressedSize} bytes (compressed)`);
    
    return this;
  }
  
  // 4. Test CPU usage (using execution time as a proxy)
  async testCpuUsage(data, iterations = 100) {
    console.log("Testing CPU usage (via execution time)...");
    
    const heavyWorkload = iterations * 10; // More iterations for CPU stress
    
    // JSON CPU usage
    const jsonStartTime = performance.now();
    for (let i = 0; i < heavyWorkload; i++) {
      const jsonData = JSON.stringify(data);
      JSON.parse(jsonData);
    }
    const jsonEndTime = performance.now();
    const jsonCpuTime = jsonEndTime - jsonStartTime;
    
    // Protobuf CPU usage
    const protobufStartTime = performance.now();
    for (let i = 0; i < heavyWorkload; i++) {
      const message = this.PersonMessage.create(data);
      const protobufData = this.PersonMessage.encode(message).finish();
      this.PersonMessage.decode(protobufData);
    }
    const protobufEndTime = performance.now();
    const protobufCpuTime = protobufEndTime - protobufStartTime;
    
    this.results.cpuUsage = {
      json: jsonCpuTime,
      protobuf: protobufCpuTime,
      difference: `${((jsonCpuTime / protobufCpuTime) * 100).toFixed(2)}%`,
      winner: jsonCpuTime < protobufCpuTime ? "JSON" : "Protobuf"
    };
    
    console.log(`JSON execution time: ${jsonCpuTime.toFixed(2)} ms`);
    console.log(`Protobuf execution time: ${protobufCpuTime.toFixed(2)} ms`);
    
    return this;
  }
  
  // 5. Test memory usage (estimating by serializing multiple times)
  async testMemoryUsage(data, iterations = 1000) {
    console.log("Testing memory usage (estimation)...");
    
    // We can't directly measure memory usage in the browser, but we can
    // create many objects and see how long garbage collection takes as a proxy
    
    // JSON memory test
    const jsonStartTime = performance.now();
    const jsonObjects = [];
    for (let i = 0; i < iterations; i++) {
      const jsonData = JSON.stringify(data);
      jsonObjects.push(JSON.parse(jsonData));
    }
    // Force garbage collection by nullifying
    const jsonLength = jsonObjects.length;
    for (let i = 0; i < jsonLength; i++) {
      jsonObjects[i] = null;
    }
    const jsonEndTime = performance.now();
    const jsonMemoryTime = jsonEndTime - jsonStartTime;
    
    // Protobuf memory test
    const protobufStartTime = performance.now();
    const protobufObjects = [];
    for (let i = 0; i < iterations; i++) {
      const message = this.PersonMessage.create(data);
      const protobufData = this.PersonMessage.encode(message).finish();
      protobufObjects.push(this.PersonMessage.decode(protobufData));
    }
    // Force garbage collection by nullifying
    const protobufLength = protobufObjects.length;
    for (let i = 0; i < protobufLength; i++) {
      protobufObjects[i] = null;
    }
    const protobufEndTime = performance.now();
    const protobufMemoryTime = protobufEndTime - protobufStartTime;
    
    this.results.memoryUsage = {
      json: jsonMemoryTime,
      protobuf: protobufMemoryTime,
      difference: `${((jsonMemoryTime / protobufMemoryTime) * 100).toFixed(2)}%`,
      winner: jsonMemoryTime < protobufMemoryTime ? "JSON" : "Protobuf"
    };
    
    console.log(`JSON memory operation time: ${jsonMemoryTime.toFixed(2)} ms`);
    console.log(`Protobuf memory operation time: ${protobufMemoryTime.toFixed(2)} ms`);
    
    return this;
  }
  
  // 6. Test network transfer time (simulation)
  async testNetworkTransfer(data, iterations = 100, latency = 50) {
    console.log("Testing network transfer time (simulation)...");
    
    // Prepare serialized data
    const jsonData = JSON.stringify(data);
    const jsonSize = new TextEncoder().encode(jsonData).length;
    
    const message = this.PersonMessage.create(data);
    const protobufData = this.PersonMessage.encode(message).finish();
    const protobufSize = protobufData.length;
    
    // Simulate network with artificial latency
    const simulateNetworkCall = (size, latency) => {
      return new Promise(resolve => {
        // Base latency + additional time based on payload size
        // Simulating ~10Mbps connection
        const transferTime = latency + (size * 8) / (10 * 1024 * 1024) * 1000;
        setTimeout(resolve, transferTime);
      });
    };
    
    // JSON network test
    const jsonStartTime = performance.now();
    for (let i = 0; i < iterations; i++) {
      await simulateNetworkCall(jsonSize, latency);
    }
    const jsonEndTime = performance.now();
    const jsonNetworkTime = (jsonEndTime - jsonStartTime) / iterations;
    
    // Protobuf network test
    const protobufStartTime = performance.now();
    for (let i = 0; i < iterations; i++) {
      await simulateNetworkCall(protobufSize, latency);
    }
    const protobufEndTime = performance.now();
    const protobufNetworkTime = (protobufEndTime - protobufStartTime) / iterations;
    
    this.results.networkTransfer = {
      json: jsonNetworkTime,
      protobuf: protobufNetworkTime,
      difference: `${((jsonNetworkTime / protobufNetworkTime) * 100).toFixed(2)}%`,
      winner: jsonNetworkTime < protobufNetworkTime ? "JSON" : "Protobuf"
    };
    
    console.log(`JSON network time: ${jsonNetworkTime.toFixed(2)} ms per request`);
    console.log(`Protobuf network time: ${protobufNetworkTime.toFixed(2)} ms per request`);
    
    return this;
  }
  
  // 7. Test latency under load
  async testLatencyUnderLoad(data, concurrentOperations = 10, iterations = 10) {
    console.log("Testing latency under load...");
    
    // Prepare serialized data
    const jsonData = JSON.stringify(data);
    const message = this.PersonMessage.create(data);
    const protobufData = this.PersonMessage.encode(message).finish();
    
    // JSON concurrent load test
    const jsonStartTime = performance.now();
    const jsonPromises = [];
    for (let i = 0; i < concurrentOperations; i++) {
      jsonPromises.push((async () => {
        for (let j = 0; j < iterations; j++) {
          JSON.parse(jsonData);
          // Add some delay to simulate real-world concurrent operations
          await new Promise(resolve => setTimeout(resolve, 1));
        }
      })());
    }
    await Promise.all(jsonPromises);
    const jsonEndTime = performance.now();
    const jsonLoadTime = jsonEndTime - jsonStartTime;
    
    // Protobuf concurrent load test
    const protobufStartTime = performance.now();
    const protobufPromises = [];
    for (let i = 0; i < concurrentOperations; i++) {
      protobufPromises.push((async () => {
        for (let j = 0; j < iterations; j++) {
          this.PersonMessage.decode(protobufData);
          // Add some delay to simulate real-world concurrent operations
          await new Promise(resolve => setTimeout(resolve, 1));
        }
      })());
    }
    await Promise.all(protobufPromises);
    const protobufEndTime = performance.now();
    const protobufLoadTime = protobufEndTime - protobufStartTime;
    
    this.results.latencyUnderLoad = {
      json: jsonLoadTime,
      protobuf: protobufLoadTime,
      difference: `${((jsonLoadTime / protobufLoadTime) * 100).toFixed(2)}%`,
      winner: jsonLoadTime < protobufLoadTime ? "JSON" : "Protobuf"
    };
    
    console.log(`JSON latency under load: ${jsonLoadTime.toFixed(2)} ms`);
    console.log(`Protobuf latency under load: ${protobufLoadTime.toFixed(2)} ms`);
    
    return this;
  }
  
  // 8. Parser initialization time (already measured in initialize method)
  reportParserInitializationTime() {
    console.log("Reporting parser initialization time...");
    
    // We'll use a negligible value for JSON since it's built-in
    const jsonInitTime = 0.01; // Negligible initialization
    
    this.results.parserInitializationTime = {
      json: jsonInitTime,
      protobuf: this.parserInitTime,
      difference: `${((jsonInitTime / this.parserInitTime) * 100).toFixed(2)}%`,
      winner: jsonInitTime < this.parserInitTime ? "JSON" : "Protobuf"
    };
    
    console.log(`JSON initialization: ${jsonInitTime.toFixed(2)} ms (built-in)`);
    console.log(`Protobuf initialization: ${this.parserInitTime.toFixed(2)} ms`);
    
    return this;
  }
  
  // 9. Test throughput
  async testThroughput(data, durationMs = 1000) {
    console.log("Testing throughput...");
    
    // JSON throughput
    let jsonCounter = 0;
    const jsonStartTime = performance.now();
    while (performance.now() - jsonStartTime < durationMs) {
      const jsonData = JSON.stringify(data);
      JSON.parse(jsonData);
      jsonCounter++;
    }
    const jsonEndTime = performance.now();
    const jsonThroughput = jsonCounter / ((jsonEndTime - jsonStartTime) / 1000);
    
    // Protobuf throughput
    let protobufCounter = 0;
    const protobufStartTime = performance.now();
    while (performance.now() - protobufStartTime < durationMs) {
      const message = this.PersonMessage.create(data);
      const protobufData = this.PersonMessage.encode(message).finish();
      this.PersonMessage.decode(protobufData);
      protobufCounter++;
    }
    const protobufEndTime = performance.now();
    const protobufThroughput = protobufCounter / ((protobufEndTime - protobufStartTime) / 1000);
    
    this.results.throughput = {
      json: jsonThroughput,
      protobuf: protobufThroughput,
      difference: `${((jsonThroughput / protobufThroughput) * 100).toFixed(2)}%`,
      winner: jsonThroughput > protobufThroughput ? "JSON" : "Protobuf"
    };
    
    console.log(`JSON throughput: ${jsonThroughput.toFixed(2)} ops/s`);
    console.log(`Protobuf throughput: ${protobufThroughput.toFixed(2)} ops/s`);
    
    return this;
  }
  
  // 10. Test schema evolution handling
  async testSchemaEvolution(data, iterations = 1000) {
    console.log("Testing schema evolution handling...");
    
    // Create evolved data by adding new fields
    const evolvedData = {
      ...data,
      additional_field: "New information",
      priority: 5
    };
    
    // Also add new field to phone numbers
    evolvedData.phones = data.phones.map(phone => ({
      ...phone,
      is_primary: phone.type === 0 // Make MOBILE phones primary
    }));
    
    // Add new field to addresses
    evolvedData.addresses = data.addresses.map(address => ({
      ...address,
      additional_info: "Extra address details"
    }));
    
    // 1. Test backwards compatibility: New schema reading old data
    
    // Serialize with original schema
    const message = this.PersonMessage.create(data);
    const originalData = this.PersonMessage.encode(message).finish();
    
    const backwardsStartTime = performance.now();
    for (let i = 0; i < iterations; i++) {
      // Deserialize with evolved schema
      this.EvolvedPersonMessage.decode(originalData);
    }
    const backwardsEndTime = performance.now();
    const backwardsTime = (backwardsEndTime - backwardsStartTime) / iterations;
    
    // 2. Test forwards compatibility: Old schema reading new data
    
    // Serialize with evolved schema
    const evolvedMessage = this.EvolvedPersonMessage.create(evolvedData);
    const evolvedSerializedData = this.EvolvedPersonMessage.encode(evolvedMessage).finish();
    
    const forwardsStartTime = performance.now();
    for (let i = 0; i < iterations; i++) {
      // Deserialize with original schema
      this.PersonMessage.decode(evolvedSerializedData);
    }
    const forwardsEndTime = performance.now();
    const forwardsTime = (forwardsEndTime - forwardsStartTime) / iterations;
    
    // For JSON, schema evolution doesn't have the same concept
    // JSON just includes all fields and clients read what they understand
    
    const jsonStartTime = performance.now();
    for (let i = 0; i < iterations; i++) {
      const jsonData = JSON.stringify(evolvedData);
      // Simulate a client that only understands original schema
      const parsed = JSON.parse(jsonData);
      const filteredData = {
        name: parsed.name,
        id: parsed.id,
        email: parsed.email,
        phones: parsed.phones.map(p => ({ number: p.number, type: p.type })),
        addresses: parsed.addresses.map(a => ({
          street: a.street,
          city: a.city,
          state: a.state,
          zip: a.zip,
          country: a.country
        })),
        metadata: parsed.metadata
      };
    }
    const jsonEndTime = performance.now();
    const jsonSchemaEvolutionTime = (jsonEndTime - jsonStartTime) / iterations;
    
    this.results.schemaEvolution = {
      json: jsonSchemaEvolutionTime,
      protobuf: {
        backwards: backwardsTime,
        forwards: forwardsTime,
        average: (backwardsTime + forwardsTime) / 2
      },
      winner: jsonSchemaEvolutionTime < (backwardsTime + forwardsTime) / 2 ? "JSON" : "Protobuf"
    };
    
    console.log(`JSON schema evolution: ${jsonSchemaEvolutionTime.toFixed(4)} ms per op`);
    console.log(`Protobuf backwards compatibility: ${backwardsTime.toFixed(4)} ms per op`);
    console.log(`Protobuf forwards compatibility: ${forwardsTime.toFixed(4)} ms per op`);
    
    return this;
  }
  
  // Run all tests
  async runAllTests(dataSize = 10) {
    const data = generateTestData(dataSize);
    console.log(`Running all tests with data size ${dataSize}...`);
    
    await this.initialize();
    await this.testSerializationSpeed(data);
    await this.testDeserializationSpeed(data);
    await this.testPayloadSize(data);
    await this.testCpuUsage(data);
    await this.testMemoryUsage(data);
    await this.testNetworkTransfer(data);
    await this.testLatencyUnderLoad(data);
    this.reportParserInitializationTime();
    await this.testThroughput(data);
    await this.testSchemaEvolution(data);
    
    console.log("All tests completed!");
    console.log("Results:", this.results);
    
    return this.results;
  }
}

// Example usage
async function runTests() {
  const tester = new PerformanceTester();
  const results = await tester.runAllTests(20); // Test with 20 items
  return results;
}

// Run and return results
runTests();