#include<stdio.h>
#include<stdlib.h>
#include<util.h>

const char* filename = "./day-7/input.txt";

const char VOID = '.';
const char BEAM = '|';
const char SPLIT = '^';
const char START = 'S';

const int MAX_COLS = 150;

long path(char** arr, int row, int col, int len);

long memo[MAX_COLS][MAX_COLS];

int main() {
  for (int r = 0; r < MAX_COLS; r++)
    for (int c = 0; c < MAX_COLS; c++)
      memo[r][c] = -1;


  int line_count = 0, line_len = 0;
  char** lines = file_lines(filename, &line_count, &line_len);
  for (int ln = 0; ln < line_count-1; ln++) {
    for (int c = 0; c < line_len; c++) {
      char* current = &lines[ln][c];
      if (*current == VOID || *current == SPLIT) continue;
      char* below = &lines[ln+1][c];
      if (*current == START) {
        *below = BEAM;
        continue;
      }

      if (*below == VOID) *below = BEAM;
      if (*below == SPLIT) {
        lines[ln+1][c-1] = BEAM;
        lines[ln+1][c+1] = BEAM;
      }
    }
  }

  int col = -1, row = -1;
  for (int ln = 0; ln < line_count; ln++) {
    for (int c = 0; c < line_len; c++) {
      if (lines[ln][c] == START) {
        row = ln;
        col = c;
        break;
      }
    }
    if (col != -1 && row != -1) break;
  }
  printf("Start: %c\n", lines[row+1][col]);
  long long sum = path(lines, row+1, col, line_count);

  printf("Sum: %lld\n", sum);
  return 0;
}

long path(char** arr, int row, int col, int len) {
  if (row == len) return 1;

  // If we already computed this cell, return cached value
  if (memo[row][col] != -1)
    return memo[row][col];

  char current = arr[row][col];

  if (current == BEAM) {
    memo[row][col] = path(arr, row+1, col, len);
    return memo[row][col];
  }

  if (current == SPLIT) {
    memo[row][col] =
      path(arr, row+1, col-1, len) +
      path(arr, row+1, col+1, len);
    return memo[row][col];
  }

  memo[row][col] = 0;
  return 0;
}


// long path(char** arr, int row, int col, int len) {
//   if (row == len) return 1;

//   char current = arr[row][col];
//   if (current == BEAM) return path(arr, row+1, col, len);
//   if (current == SPLIT) return path(arr, row+1, col-1, len) + path(arr, row+1, col+1, len);
//   return 0;
// }