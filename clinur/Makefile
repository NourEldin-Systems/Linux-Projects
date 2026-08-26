# Changed from CXX to CC for the C compiler
CC = gcc
# Changed CXXFLAGS to CFLAGS and updated standard to c11 (or c17/c99 depending on your needs)
CFLAGS = -O3 -std=gnu11 -Wall

SRC_DIR = src
INC_DIR = include
LIB_DIR = lib
BUILD_DIR = build

# Changed extension from .cpp to .c
SRCS = $(shell find $(SRC_DIR) -name "*.c")
OBJS = $(patsubst $(SRC_DIR)/%.c, $(BUILD_DIR)/%.o, $(SRCS))
DEPS = $(OBJS:.o=.d)

TARGET = $(BUILD_DIR)/clinur

all: prepare $(TARGET)

prepare:
	@mkdir -p $(BUILD_DIR)

$(TARGET): $(OBJS)
	$(CC) $(OBJS) -o $@ $(LDFLAGS)

# Updated the pattern rule to look for .c files and use $(CC) / $(CFLAGS)
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(DEPFLAGS) -I$(INC_DIR) -I$(LIB_DIR) -c $< -o $@

-include $(DEPS)

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all prepare clean
