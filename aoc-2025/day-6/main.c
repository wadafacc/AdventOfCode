#include<stdio.h>
#include<stdlib.h>
#include<util.h>

const int MULT = -2;
const int ADD = -1;


int main() { 
  char* input = "./day-6/input.txt";

  FILE* fp;
  char* line = NULL;
  size_t _len = 0;

  fp = fopen(input, "r");
  if (fp == NULL) {
    perror("error opening file");
    return 1;
  }

  int line_count = get_lines(input);
  int** lines = malloc(line_count * sizeof(int*));
  int i = 0;
  while (getline(&line, &_len, fp) != -1) {
    int len = str_len(line);
    combine(&line, &len, ' ');
    int arr_len = count_char(line, len, ' ');
    char** arr = str_split(line, len, ' ');

    lines[i] = (int*)malloc(arr_len * sizeof(int));
    for (int j = 0; j < arr_len; j++) {
      int n = atoi(arr[j]);
      if (n > 0) {
        lines[i][j] = n;
      } else {
        if (contains(arr[j], 5, '*')) lines[i][j] = MULT;
        if (contains(arr[j], 5, '+')) lines[i][j] = ADD;
      }
    }
    i++;
  }

  int problem_count = 0, ii = 0;
  while (lines[line_count-1][ii] != 0) {
     problem_count++;
     ii++;
  };
  printf("Problem Count: %d\n", problem_count);

  long total = 0;
  for (int i = 0; i < problem_count; i++) {
    int mode = 0; // add, mult
    long sum = 0; // sum of problem
    for (int j = line_count-1; j >= 0; j--) {
      int number = lines[j][i];
      if (number == 0) continue;
      if (number < 0) { mode = number; continue; }  
      printf("%d ", number);
      if (sum == 0) { sum = (long)number; continue; }

      if (mode == MULT) sum *= (long)number;
      if (mode == ADD) sum += (long)number;
    }
    printf("Sum: %d\n", sum);
    total += sum;
  }
  printf("Total: %ld\n", total);
  return 0;
}