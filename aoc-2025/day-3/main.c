#include<stdio.h>
#include<stdlib.h>

int arr_len(char* a);
int char_to_int(char n);
char int_to_char(int n);

int main() {
  FILE* fp;
  char* ln = NULL; 
  size_t len = 0;

  fp = fopen("./day-3/input.txt", "r");
  if (fp == NULL) {
    perror("open fail");
    exit(EXIT_FAILURE);
  }

  int sum = 0;
  while (getline(&ln, &len, fp) != -1) {
    printf("%s", ln);
    int max = 0;
    for (int i = 0; i < arr_len(ln); i++) {
      int n = char_to_int(ln[i]);
      for (int j = i+1; j < arr_len(ln); j++) {
        int m = char_to_int(ln[j]);
        int nm = (n * 10) + m;
        if (nm > max) max = nm;
      }
    }
    sum += max;
  }

  printf("\nSum: %d", sum);
  fclose(fp);
}

int arr_len(char* a) {
  int i = 0;
  while (a[i] != '\0') i++;
  return i;
} 

int char_to_int(char n) {
  return (int)(n - '0');
}

char int_to_char(int n) {
  return (char)(n + '0');
}