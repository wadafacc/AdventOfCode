#include<stdio.h>
#include<stdlib.h>

int arr_len(char* a);
int char_to_int(char n);
char int_to_char(int n);
long sum_arr(char* arr);
void del(char* arr, int *n, int key);
long _pow(int b, int e);

const int MAX_LEN = 12;

int main() {
  FILE* fp;
  char* line = NULL; 
  size_t len = 0;

  fp = fopen("./day-3/input.txt", "r");
  if (fp == NULL) {
    perror("open fail");
    exit(EXIT_FAILURE);
  }

  long sum = 0;
  while (getline(&line, &len, fp) != -1) {
    printf("%s", line);

    // get first biggest num
    int xlen = arr_len(line);
    int idx = -1;

    char num[MAX_LEN] = { '0' };
    for (int n = 0; n < MAX_LEN; n++) {
      for (int i = idx + 1; i < (xlen - 2 - (10 - n)); i++) {
        if (line[i] > num[n]) {
          num[n] = line[i];
          idx = i;
        }
      }
    }
    sum += sum_arr(num);
  }

  printf("Sum: %ld", sum);
  fclose(fp);
}

int arr_len(char* a) {
  int i = 0;
  while (a[i] != '\0') i++;
  return i;
} 

long sum_arr(char* arr) {
  long sum = 0;
  int len = arr_len(arr);
  for (int i = 0; i < len-1; i++) {
    sum += char_to_int(arr[i]) * _pow(10, len-i-2);
    printf("%d", char_to_int(arr[i]));
  }
  printf("\n");
  return sum;
}

int char_to_int(char n) {
  return (int)(n - '0');
}

char int_to_char(int n) {
  return (char)(n + '0');
}

void del(char* arr, int *n, int idx) {
  for (int j = idx; j < *n - 1; j++) {
    arr[j] = arr[j + 1];
  }
  // Decrease the size
  (*n)--; 
}

long _pow(int b, int e) {
  long sum = 1;
  for (int i = 0; i < e; i++) {
    sum *= b;
  }
  return sum;
}