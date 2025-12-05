#include<stdio.h>
#include<stdlib.h>


int len(char* arr);
int get_lines(char* filename);
int contains(long* arr, int len, long item);
void add(long** arr, int* size, int* len, long item);
int range_comp(long* r1, long* r2);
void del(long ***arr, int *len, int idx);

int comp(const void* a, const void* b) {
  const long* ra = *(const long**)a;
  const long* rb = *(const long**)b;
  
  if (ra[0] < rb[0]) return -1;
  if (ra[0] > rb[0]) return 1;
  return 0;
}

long min(long a, long b) {
  if (a < b ) return a;
  return b;
}
long max(long a, long b) {
  if (a > b) return a;
  return b;
}

int main() {
  FILE* fp;
  char* line = NULL;
  size_t _len = 0;  // not really used

  fp = fopen("./input.txt", "r");
  if (fp == NULL) {
    perror("Error opening file");
    return 1;
  }

  // get all ranges
  int bound_count = get_lines("./input.txt");
  long** ranges = (long**)malloc(bound_count * sizeof(long*));
  long i = 0;
  while ((getline(&line, &_len, fp) != -1) && len(line) != 0) {
    long f = 0, t = 0;
    sscanf(line, "%ld-%ld", &f,&t);

    ranges[i] = (long*)malloc(2 * sizeof(long));
    ranges[i][0] = f;
    ranges[i][1] = t;
    i++;
  }
  
  qsort(ranges, bound_count, sizeof(long*), comp); // sort asc

  i = 0;
  int len = bound_count;
  while (i < len-1) {
    long* a = ranges[i];
    long* b = ranges[i+1];
    printf("%ld-%ld | %ld-%ld\n", a[0], a[1], b[0], b[1]);
    if (range_comp(a,b) == 1) {
      long* merged = malloc(2 * sizeof(long));
      merged[0] = min(a[0], b[0]);
      merged[1] = max(a[1], b[1]);

      ranges[i] = merged;
      del(&ranges, &len, i+1);
      continue;
    }    
    // nothing to do here
    i++;
  }

  for (long i = 0; i < len; i++) {
    printf("%ld, %ld\n",ranges[i][0], ranges[i][1]);
  }
  long sum = 0;
  for (int i = 0; i < len; i++) {
    sum += (ranges[i][1] - ranges[i][0] ) +1;
  }

  printf("Sum: %ld", sum);
}

int range_comp(long* r1, long* r2) {
  long a1 = r1[0], a2 = r1[1];
  long b1 = r2[0], b2 = r2[1];

  // 0: no overlap
  if (a2 < b1 || a1 > b2)
    return 0;

  return 1;
}


int len(char* arr) {
  int c = 0;
  while (arr[c] != '\0' && arr[c] != '\n') c++;
  return c;
}

// counts until empty line
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

  while ((getline(&line, &_len, fp) != -1) && len(line) != 0) c++;
  free(line);
  
  return c;
}

int contains(long* arr, int len, long item) {
  for (int i = 0; i < len; i++) {
    if (arr[i] == item) return 1;
  }
  return 0;
}

void add(long** arr, int* size, int* len, long item) {
  if (*size == *len) {
    *size *= 2;
    long* temp = realloc(*arr, (*size) * sizeof(long));
    if (!temp) return; // handle allocation failure
    *arr = temp;
  }
  (*arr)[*len] = item;
  (*len)++;
}

void del(long ***arr, int *len, int idx) {
  // free the row you are deleting
  free((*arr)[idx]);

  // shift pointers left
  for (int i = idx; i < *len - 1; i++) {
    (*arr)[i] = (*arr)[i + 1];
  }

  // shrink pointer array
  long **temp = realloc(*arr, (*len - 1) * sizeof(long*));
  if (temp == NULL && *len - 1 > 0) return;

  *arr = temp;
  (*len)--;
}
