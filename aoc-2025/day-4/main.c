#include <stdio.h>
#include <stdlib.h>

int arr_len(char* ln);
int get_lines(char* filename);
int check_neighbors(char** arr, int rows, int cols, int x, int y);

const char PAPER = '@';

int main() {
  FILE* fp;
  char* line = NULL;
  size_t len = 0;

  fp = fopen("./input.txt", "r");
  if (fp == NULL) {
    perror("Error opening file");
    return 1;
  }
  int ln = get_lines("./input.txt");

  
  // alloc arr2
  char** arr = (char**)malloc(ln * sizeof(char*));
  char** state = (char**)malloc(ln * sizeof(char*));
  int i = 0;
  while (getline(&line, &len, fp) != -1) {
    int len = arr_len(line);
    arr[i] = (char*)malloc(len * sizeof(char));
    state[i] = (char*)malloc(len * sizeof(char));
    for (int c = 0; c < len; c++) {
      arr[i][c] = line[c];
    }
    arr[i][len] = '\0';
    i++;
  }

  int rl = ln;  // row len
  int cl = arr_len(arr[0]); // col len
  int sum = 0, nsum = 0;
  do {
    nsum = 0;
    for (int y = 0; y <= rl; y++) {
      for (int x = 0; x < cl-1; x++) {
        if (arr[y][x] == '.') {
          printf(".");
          state[y][x] = '.';
          continue;
        }
        int neighbors = check_neighbors(arr, rl, cl, x, y);
        if (neighbors < 4) {
          nsum++;
          printf("x");
          state[y][x] = '.';
        } else {
          printf("%c", arr[y][x]);
          state[y][x] = arr[y][x];
        } 
      }
      printf("\n");
      state[y][cl] = '\0';  // important for arr_len
    }
    sum += nsum;
    arr = state;
  } while (nsum != 0);
  printf("\n%d", sum);

  free(line);
  fclose(fp);
  return 0;
}

int arr_len(char* ln) {
  int i = 0;
  while (ln[i] != '\0') i++;
  return i;
}

int get_lines(char* filename) {
  FILE* fp;
  fp = fopen(filename, "r");
  if (fp == NULL) {
    perror("Error opening file");
    return -1;
  }
  int ch, lines = 0;
  while ((ch = fgetc(fp)) != EOF) {
    if (ch == '\n') lines++;
  }
  return lines;
}

//returns adjacent @'s 
int check_neighbors(char** arr, int rows, int cols, int x, int y) {
  int count = -1; // since it counts itself aswell
  for (int i = y-1; i <= y+1; i++) {
    for (int j = x-1; j <= x+1; j++) {
      if ((i < 0 || i > rows) || (j < 0 || j > cols)) continue; // skip if OUB
      if (arr[i][j] == PAPER) count++;
    }
  }

  return count;
}