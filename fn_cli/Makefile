CXX = g++
CXXFLAGS = -O3 -std=c++2b -Wall
DEPFLAGS = -MMD -MP

SRC_DIR = src
INC_DIR = include
BUILD_DIR = build

# FIX 1: Added quotes around '*.cpp' so the shell find command works reliably
SRCS = $(shell find $(SRC_DIR) -name '*.cpp')
OBJS = $(patsubst $(SRC_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(SRCS))
DEPS = $(OBJS:.o=.d)

TARGET = $(BUILD_DIR)/fn_cli

all: prepare $(TARGET)

prepare:
	@mkdir -p $(BUILD_DIR)

$(TARGET): $(OBJS)
	$(CXX) $(OBJS) -o $@ $(LDFLAGS)

# FIX 2: Removed the undefined -I$(LIB_DIR) unless you choose to add LIB_DIR = lib
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) $(DEPFLAGS) -I$(INC_DIR) -c $< -o $@

-include $(DEPS)

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all prepare clean
