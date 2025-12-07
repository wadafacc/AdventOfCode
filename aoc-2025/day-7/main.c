#include<stdio.h>
#include<stdlib.h>
#include<util.h>

const char* filename = "./day-7/input.txt";

const char VOID = '.';
const char BEAM = '|';
const char SPLIT = '^';
const char START = 'S';

int main() {
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

  int sum = 0;
  for (int ln = 1; ln < line_count; ln++) {
    printf("%s\n", lines[ln]);
    for (int c = 0; c < line_len; c++) {
      char* current = &lines[ln][c];
      if (*current != SPLIT) continue;
      char above = lines[ln-1][c];
      if (above == BEAM) sum++;
    }
  }

  printf("Sum: %d\n", sum);
  return 0;
}