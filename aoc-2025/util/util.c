#include<stdio.h>
#include<stdlib.h>
#include<util.h>

int str_len(char* str) {
  int i = 0;
  while (str[i] != '\0' && str[i] != '\n') i++;
  return i;
}

int get_lines(char* filename) {
  FILE* fp;
  char* line = NULL;
  size_t _len = 0;  // not really used

  int c = 0;
  fp = fopen(filename, "r");
  if (fp == NULL) {
    perror("Error opening file");
    return -1;
  }

  while ((getline(&line, &_len, fp) != -1) && str_len(line) != 0) c++;
  free(line);
  
  return c;
}


void combine(char** str, int* len, char c) {
  int i = 0;
  while (i < (*len)-2) {
    if ((*str)[i] == c && (*str)[i+1] == c) {
      del(str, len, i);
      continue;
    }
    i++;
  }
  char* temp = realloc(*str, ((*len)+1) * sizeof(char));
  if (temp == NULL) return;
  *str = temp;
}

void del(char** arr, int* len, int idx) {
  for (int i = idx; i < (*len)-1; i++) {
    (*arr)[i] = (*arr)[i+1];
  }
  // reduce len for realloc afterwards
  (*len)--;
}

void trim(char** str, int* len) {
  int i = 0;
  while ((*str)[i] == ' ') {
    del(str, len,i);
    i++;
  }
  i = *len;
  while ((*str)[i] == ' ' || (*str)[i] == '\n') {
    del(str, len,i);
    i--;
  }

  char* temp = realloc(*str, ((*len)+1) * sizeof(char));
  if (temp == NULL) return;
  *str = temp;
  (*str)[*len] = '\0'; // Manually null-terminate the shortened string
}

int contains(char* str, int len, char c) {
  for (int i = 0; i < len; i++) {
    if (str[i] == c) return 1;
  }
  return 0;
}

/* 
STRING SPLIT STUFF
*/
int chars_till_delim(char* str, int start, int len, char delim) {
  if (len == 0) return -1;
  int c = 0;
  for (int i = start; i < len; i++) {
    if (str[i] == delim) return c;
    c++;
  }
  return c;
}

int count_char(char* str, int len, char c) {
  int count = 0;
  for (int i = 0; i < len; i++) {
    if (str[i] == c) count++;
  }
  return count;
}

char** str_split(char* str, int len, char delim) {
  int delim_count = count_char(str, len, delim)+1;
  char** arr = malloc(delim_count * sizeof(char*));

  int prev_idx = 0, c = 0;
  for (int i = 0; i < len; i++) {
    if (str[i] != delim) continue;
    if (str[i] == delim && (str[i-1] == delim || i == prev_idx || str[i+1] == '\0')) continue; // prevent empty elements?

    int l = i-prev_idx +1;
    arr[c] = (char*)malloc(l * sizeof(char));
    str_cpy(str, &arr[c], prev_idx, l);
    prev_idx = i;
    c++;
  }
  arr[c] = (char*)malloc((len+1-prev_idx) * sizeof(char));
  str_cpy(str, &arr[c], prev_idx, len-prev_idx);

  return arr;
}

void str_cpy(char* src, char** dst, int start, int len) {
  for (int i = 0; i < len; i++) {
    (*dst)[i] = src[i+start];
  }
  (*dst)[len-1] = '\0';
}

