
CXX = g++
CXXFLAGS = -O3 -std=c++2b -Wall
DEPFLAGS = -MMD -MP
# LDFLAGS  = -lws2_32

SRC_DIR = src
INC_DIR = include
LIB_DIR = lib
BUILD_DIR = build

SRCS = $(shell find $(SRC_DIR) -name "*.cpp")
OBJS = $(patsubst $(SRC_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(SRCS))
DEPS = $(OBJS:.o=.d)

TARGET = $(BUILD_DIR)/nourzz-life.exe

all: prepare $(TARGET)

prepare:
	@mkdir -p $(BUILD_DIR)

$(TARGET): $(OBJS)
	$(CXX) $(OBJS) -o $@ $(LDFLAGS)

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) $(DEPFLAGS) -I$(INC_DIR) -I$(LIB_DIR) -c $< -o $@

-include $(DEPS)

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all prepare clean
