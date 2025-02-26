# CMakeLists.txt
cmake_minimum_required(VERSION 3.10)
project(JsonVsProtobufBenchmark)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3")

# Find dependencies
find_package(Protobuf REQUIRED)
find_package(nlohmann_json 3.9.1 REQUIRED)
find_package(ZLIB REQUIRED)
find_package(Threads REQUIRED)

# Generate protocol buffer files
protobuf_generate_cpp(PROTO_SRCS PROTO_HDRS person.proto person_evolved.proto)

# Include directories
include_directories(
    ${CMAKE_CURRENT_BINARY_DIR}
    ${Protobuf_INCLUDE_DIRS}
)

# Add executable
add_executable(benchmark 
    benchmark.cpp
    ${PROTO_SRCS}
    ${PROTO_HDRS}
)

# Link libraries
target_link_libraries(benchmark
    ${Protobuf_LIBRARIES}
    nlohmann_json::nlohmann_json
    ZLIB::ZLIB
    Threads::Threads
)