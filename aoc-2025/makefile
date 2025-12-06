# Use clang on macOS (or gcc on Linux)
CC := gcc
CFLAGS := -Wall -Wextra -O2 -I. -Iutil # Include both the root (for main.c) and the util folder (for util.h if needed)

# Default folder - **Use := instead of ?=**
DAY := 6 

# --- File Definitions ---

# Source files:
MAIN_SRC := day-$(DAY)/main.c
UTIL_SRC := util/util.c
ALL_SRCS := $(MAIN_SRC) $(UTIL_SRC)

# Output executable file location:
OUT := day-$(DAY)/main.out

# --- Targets ---

all: run

# Compile and link all source files into the final executable $(OUT)
$(OUT): $(ALL_SRCS)
	$(CC) $(CFLAGS) $(ALL_SRCS) -o $(OUT)

# Run the program
run: $(OUT)
	@$(OUT)

# Clean target to remove generated files
clean:
	rm -f $(OUT)

.PHONY: all run clean