CXX 	 := g++
CXXFLAGS := -Wall -Wextra -std=c++20 -pthread

BIN := bin
SRC := src
EXEC := main

all: $(BIN)/$(EXEC)

run: clean all
	clear
	@echo "Executing..."
	./$(BIN)/$(EXEC)

$(BIN)/$(EXEC): $(SRC)/*.cpp
	@echo "Building..."
	$(CXX) $(CXXFLAGS) $^ -o $@

clean:
	@echo "Cleaning..."
	-rm $(BIN)/*
