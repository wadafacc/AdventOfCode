#include<stdio.h>
#include<stdlib.h>

#include<util.h>


typedef struct Edge {int a,b; long long distance; } Edge;
const char* filename = "./day-8/input.txt";
int LIMIT = 1000;

int* line_to_point(char* line);
// calculate distance between points
long long calc_dist(int* p, int* q);
// DSU, find.
int find(int x);
// DSU: set parent
void set_parent(int a, int b);

int cmp_dist(const void *a, const void *b) {
  const Edge *ea = (const Edge*)a;
  const Edge *eb = (const Edge*)b;
  if (ea->distance < eb->distance) return -1;
  if (ea->distance > eb->distance) return 1;
  return 0;
}

int cmp_size(const void *a, const void *b) {
  int x = *(const int*)a;
  int y = *(const int*)b;
  return y - x;  // descending
}

int* parent;
int* size;

int main() {
  int line_count = 0, line_len = 0;
  char** lines = file_lines(filename, &line_count, &line_len);
  int** coords = malloc(line_count * sizeof(int*));
  for (int i = 0; i < line_count; i++) {
    coords[i] = line_to_point(lines[i]);
  }

  parent = malloc(line_count * sizeof(int));
  size = malloc(line_count * sizeof(int));
  for (int i = 0; i < line_count; i++) {
    parent[i] = i;  // set everything to its own parent
    size[i] = 1;
  }

  int count = line_count * (line_count - 1) / 2;
  Edge* map = malloc(count * sizeof(Edge));

  int k = 0;
  for (int i = 0; i < line_count; i++) {
    for (int j = i+1; j < line_count; j++) {
      map[k].a = i;
      map[k].b = j;
      map[k].distance = calc_dist(coords[i], coords[j]);
      k++;
    }
  }

  qsort(map, count, sizeof(Edge), cmp_dist);

  if (LIMIT > count) LIMIT = count; // just in case there are fewer than 1000 edges

  for (int i = 0; i < LIMIT; i++) {
    int a = map[i].a;
    int b = map[i].b;
    if (find(a) != find(b)) {
      set_parent(a, b);
    }
  }

  int* seen = calloc(line_count, sizeof(int));
  int* set_sizes = malloc(line_count * sizeof(int));
  int n_sizes = 0;

  for (int i = 0; i < line_count; i++) {
    int root = find(i);
    if (!seen[root]) {
      set_sizes[n_sizes++] = size[root];
      seen[root] = 1;
    }
  }

  qsort(set_sizes, n_sizes, sizeof(int), cmp_size);

int top3_product = set_sizes[0] * set_sizes[1] * set_sizes[2];
  printf("Top 3 sizes: %d, %d, %d -> product = %d\n",
    set_sizes[0], set_sizes[1], set_sizes[2], top3_product);


  return 0;
}

int find(int x) {
  if (parent[x] != x)
    parent[x] = find(parent[x]); // path compression
  return parent[x];
}

void set_parent(int a, int b) {
  int rootA = find(a);  // find the root of a's set
  int rootB = find(b);  // find the root of b's set
  if (rootA != rootB) {
    parent[rootB] = rootA;  // attach B's root to A's root
    size[rootA] += size[rootB];
  }
}


int* line_to_point(char* line) {
  int* coords = malloc(3 * sizeof(int));
  sscanf(line, "%d,%d,%d", &coords[0], &coords[1], &coords[2]);
  return coords;
}

// -> d^2 = ((p-q)^2 + (p-q)^2 + (p-q)^2)
long long calc_dist(int* p, int* q) {
  long long x = p[0] - q[0];
  long long y = p[1] - q[1];
  long long z = p[2] - q[2];

  return ((x*x) + (y*y) + (z*z));
}