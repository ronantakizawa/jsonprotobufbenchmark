CXX = g++
CXXFLAGS = -std=c++17 -O3 -Wall

all: benchmark

benchmark: benchmark.cpp
	$(CXX) $(CXXFLAGS) -o benchmark benchmark.cpp

clean:
	rm -f benchmark
	
.PHONY: all clean