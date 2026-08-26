CXX      := g++
CXXFLAGS := -Wall -Wextra -std=c++17 -Iinclude
LIBS     := -lraylib -lGL -lm -lpthread -ldl -lrt -lX11

SRC_DIR  := src
BUILD_DIR:= build
TARGET   := bin/game

# Find all .cpp files inside src/
SRCS     := $(wildcard $(SRC_DIR)/*.cpp)
OBJS     := $(SRCS:$(SRC_DIR)/%.cpp=$(BUILD_DIR)/%.o)

all: $(TARGET)

# Link object files into executable
$(TARGET): $(OBJS) | bin
	$(CXX) $(OBJS) -o $@ $(LIBS)

# Compile .cpp files into .o object files
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Create directories if they don't exist
$(BUILD_DIR) bin:
	mkdir -p $@

run: all
	./$(TARGET)

clean:
	rm -rf $(BUILD_DIR) bin

.PHONY: all run clean
