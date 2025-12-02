#include<stdio.h>
#include<stdlib.h>
#include<math.h>

// count delimiters in string for memory allocation
int count_delim(char* str, int len, char delim);
// counts characters until delim for memalloc
int chars_till_delim(char* str, int start, int len, char delim);
// split string by delim
char** str_split(char*str, int len, char delim);
// get string length
int str_len(char* str);
// get number length
int num_len(int x);
// get slice of string
void slice(const char *src, char *dst, int start, int len);
// repeat a substring n times
void repeat(char* substr, char* dst, int n);
// string comparison
int str_cmp(char* this, char* that);

int main() {
  FILE * fp;
  char * line = NULL;
  size_t len = 0;

  fp = fopen("./day-2/input.txt", "r");
  if (fp == NULL) {
    perror("open fail");
    exit(EXIT_FAILURE);
  }
  
  getline(&line, &len, fp);
  int real_len = str_len(line);
  char** list = str_split(line, real_len, ',');
  
  long sum = 0;

  for (int i = 0; i < count_delim(line, real_len, ',')+1; i++) {
    printf("--- %s ---\n", list[i]);
    int n1,n2;
    sscanf(list[i], "%d-%d", &n1, &n2);

    for (int i = n1; i <= n2; i++) {
      // window loop
      int l = num_len(i);
      // if ((l % 2) != 0) continue; // only need to care about even number lengths

      char* num = (char*)malloc((l+1) * sizeof(char));
      sprintf(num, "%d", i);

      for (int w = 1; w <= l/2; w++) {
        // if (l % w != 0) continue;  // Window size must divide evenly
        char* v = (char*)malloc((l+1) * sizeof(char));
        char* s = (char*)malloc((w+1) * sizeof(char));
        slice(num,s,0,w);

        repeat(s, v, l/w);
        // printf("%s, %s \n", num, v);
        int comp = str_cmp(num, v); 
        if (comp == 1) {
          printf("%s -> invalid\n", v);
          sum += i;
          break;
        }
      }
    }
  }
  printf("Sum: %lld\n", sum);

  fclose(fp);
  if (line) {
    free(line);
  }
}

int count_delim(char* str, int len, char delim) {
  int c = 0;
  for (int i = 0; i < len; i++) {
    if (str[i] == delim) c++;
  }
  return c;
}

char** str_split(char*str, int len, char delim) {
  if (len == 0) return 0;

  int n_delim = count_delim(str, len, delim);
  char** out = malloc((n_delim+1) * sizeof(char*));
  int c = 0;
  int prev_idx = 0;
  out[c] = (char*)malloc((chars_till_delim(str, 0, len, delim)+1) * sizeof(char));
  
  for (int i = 0; i < len; i++) {
    if (str[i] == delim) {
      out[c][i - prev_idx] = '\0';  // Fixed
      prev_idx = i+1;
      c++;
      out[c] = (char*)malloc((chars_till_delim(str, i+1, len, delim)+1) * sizeof(char));
      continue;
    }
    out[c][i - prev_idx] = str[i];
  }
  out[c][len - prev_idx] = '\0';  // Added: null-terminate last string
  
  return out;
}


int chars_till_delim(char* str, int start, int len, char delim) {
  if (len == 0) return -1;
  int c = 0;
  for (int i = start; i < len; i++) {
    if (str[i] == delim) return c;
    c++;
  }
  return c;
}

int str_len(char* str) {
  int c = 0;
  while (str[c] != '\0') c++;
  return c;
}

int num_len(int x) {
  if (x == 0) return 1;
  return floor(log10(abs (x))) + 1;
}

void slice(const char *src, char *dst, int start, int len) {
  for (int i = 0; i < len; i++) {
    dst[i] = src[start + i];
  }
  dst[len] = '\0';
}

void repeat(char* substr, char* dst, int n) {
  int substr_len = str_len(substr);
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < substr_len; j++) {
      dst[(i * substr_len) + j] = substr[j];
    }
  }
  dst[n * substr_len] = '\0';
}

int str_cmp(char* this, char* that) {
  int i = 0;
  while (this[i] != '\0' && that[i] != '\0') {
    if (this[i] != that[i]) return -1;
    i++;
  }
  // Check if both strings ended (both hit '\0')
  return (this[i] == '\0' && that[i] == '\0') ? 1 : -1;
}