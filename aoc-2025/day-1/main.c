#include<stdio.h>
#include<stdlib.h>


const char LEFT = 'L';
const char RIGHT = 'R';


// splits a string at idx into 2 parts
char** str_split_once(char * str, int idx, int len);

int main() {
  FILE * fp;
  char * line = NULL;
  size_t len = 0;
  ssize_t read;
  
  fp = fopen("./day-1/input.txt", "r");
  if (fp == NULL) {
    perror("open fail");
    exit(EXIT_FAILURE);
  }
  
  int s = 50;
  int c = 0; // counter
  while ((read = getline(&line, &len, fp)) != -1) {
    char** out = str_split_once(line, 1, len);
    int a;
    sscanf(out[1], "%d", &a);
    if (*out[0] == RIGHT) {
      s += a;
    } else {
      s -= a;
    }

    s = (s + 100) % 100;
    if (s == 0) c++;
  }

  printf("END: %d\n", c);

  fclose(fp);
  if (line) {
    free(line);
  }
}

char** str_split_once(char * str, int idx, int len) {
  char** out = malloc(2 * sizeof(char*));

  out[0] = (char*)malloc(2); // char + terminator
  out[0][0] = str[0];
  out[0][1] = '\0';

  out[1] = (char*)malloc(len);  // terminator included, since i = 1
  for (int i = 0; i < len; i++) {
    out[1][i] = str[i + idx];
  }
  out[1][len] = '\0';
  return out; 
}