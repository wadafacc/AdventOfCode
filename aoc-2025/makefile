# Use clang on macOS (or gcc on Linux)
CC := gcc
CFLAGS := -Wall -Wextra -O2

# Default folder - **Use := instead of ?=**
DAY := .

SRC := day-$(DAY)/main.c
OUT := day-$(DAY)/main.o

all: run

# Compile main.c
$(OUT): $(SRC)
	$(CC) $(CFLAGS) -o $(OUT) $(SRC)

# Run the program
run: $(OUT)
	@$(OUT)