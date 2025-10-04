CXX 	 := g++
CXXFLAGS := -Wall -Wextra -std=c++20 -pthread

BIN  	  := bin
SRC  	  := src
EXEC 	  := main
LIB       := lib
INCLUDE   := include
LIBRARIES := -lpigpio -lrt -ltdtp

all: $(BIN)/$(EXEC)

run: clean all
	clear
	@echo "Executing..."
	./$(BIN)/$(EXEC)

$(BIN)/$(EXEC): $(SRC)/*.cxx
	@echo "Building"
	$(CXX) $(CXXFLAGS) -I$(INCLUDE) -L$(LIB) $^ -o $@ $(LIBRARIES)

clean:
	@echo "Cleaning..."
	-rm $(BIN)/*
