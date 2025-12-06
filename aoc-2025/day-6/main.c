#include<stdio.h>
#include<stdlib.h>
#include<util.h>

const int MULT = (int)'*';
const int ADD = (int)'+';
const char* INVALID = "    ";

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
  char** lines = malloc(line_count * sizeof(char*));
  int i = 0, longest = 0;
  while (getline(&line, &_len, fp) != -1) {
    int len = str_len(line);
    lines[i] = malloc((len+1) * sizeof(char));
    str_cpy(line, &lines[i], 0, len+1);
    if (len > longest) longest = len;
    printf("Line: '%s'\n", lines[i]);
    i++;
  }
  printf("\n");

  long sum = 0;
  int mode = 0;
  long total = 0;
  for (int i = 0; i < longest; i++) {
    char* num = malloc((line_count-1) * sizeof(char));
    if (lines[line_count-1][i] != ' '){
      mode = (int)lines[line_count-1][i];
      printf("Mode: '%c'\n", (char)mode);
    }

    for (int j = 0; j < line_count-1; j++) {
      num[j] = lines[j][i];
    }

    long number = atoi(num);
    printf("%d ", number);
    if (number == 0) { // invalid number -> ' '
      printf("= %d\n", sum);
      total += sum;
      sum = 0;
      continue; 
    }
    if (sum == 0) {
      sum = number;
      continue;
    }

    if (mode == MULT) sum *= number;
    if (mode == ADD) sum += number;
  }
  printf("\nTotal: %ld\n", total);

  return 0;
}