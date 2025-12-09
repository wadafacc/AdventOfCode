#include<stdio.h>
#include<stdlib.h>
#include<util.h>
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

const char* filename = "./day-9/input.txt";
const int VALID = 1;
const int EMPTY = 1;

typedef struct Coord {int x,y; } Coord;

long part_01(Coord* coords, int line_count);
long part_02(Coord* coords, int line_count);


Coord str_to_coord(char* str);
long area(Coord p, Coord q);
void flood_fill(int*** arr, int x, int y, int width, int height);

int main() {
  int line_count = 0, line_len = 0;
  char** lines = file_lines(filename, &line_count, &line_len);
  Coord* coords = malloc(line_count * sizeof(Coord));
  for (int i = 0; i < line_count; i++) {
    coords[i] = str_to_coord(lines[i]);
  }


  printf("Part 1: %ld\n", part_01(coords, line_count));
  printf("Part 2: %ld\n", part_02(coords, line_count));
}

long part_01(Coord* coords, int line_count) {
  long biggest_a = 0;
  for (int i = 0; i < line_count; i++) {
    for (int j = i+1; j < line_count; j++) {
      long c_area = area(coords[i], coords[j]);
      if (c_area > biggest_a) biggest_a = c_area;
    }
  }

  return biggest_a;
}

long part_02(Coord* coords, int line_count) {
  int** valid_tiles;
  int min_x = coords[0].x, min_y = coords[0].y;
  int max_x = 0, max_y = 0;

  for (int i = 0; i < line_count; i++) {
    min_x = MIN(min_x, coords[i].x);
    min_y = MIN(min_y, coords[i].y);
    max_x = MAX(max_x, coords[i].x);
    max_y = MAX(max_y, coords[i].y);
  }

  int w = max_x - min_x + 1;
  int h = max_y - min_y + 1;
  valid_tiles = malloc(h * sizeof(int*));
  for (int i = 0; i < h; i++) {
    valid_tiles[i] = calloc(w, sizeof(int));  // make sure to 0 rows themselves for floodfill afterwards
  }

  // init boundaries
  for (int i = 0; i < line_count; i++) {
    int next_idx = (i+1) % line_count;

    int ox = coords[i].x - min_x;
    int oy = coords[i].y - min_y;
    valid_tiles[oy][ox] = VALID;

    // fill in edges
    int x1 = coords[i].x, x2 = coords[next_idx].x;
    int y1 = coords[i].y, y2 = coords[next_idx].y;

    for (int dy = MIN(y1,y2); dy <= MAX(y1,y2); dy++) {
      for (int dx = MIN(x1,x2); dx <= MAX(x1,x2); dx++) {
        valid_tiles[dy - min_y][dx - min_x] = VALID;
      }
    }
  }

  // find start position for flood fill
  Coord start;
  start.x = (coords[0].x - min_x) + 1;  // One tile right of first red tile
  start.y = coords[0].y - min_y+1;

  flood_fill(&valid_tiles, start.x, start.y, w, h);
  // TODO: check valid rects

  // for (int i = 0; i < h; i++) {
  //   for (int j = 0; j < w; j++) {
  //     printf("%d", valid_tiles[i][j]);
  //   }
  //   printf("\n");
  // }

  return 0;
}

void flood_fill(int*** arr, int x, int y, int width, int height) {
  if (x < 0 || x > width || y < 0 || y > height) return;

  if ((*arr)[y][x] == VALID) return;
  (*arr)[y][x] = VALID;
  flood_fill(arr, x-1, y, width, height);
  flood_fill(arr, x+1, y, width, height);
  flood_fill(arr, x, y-1, width, height);
  flood_fill(arr, x, y+1, width, height);
  return;
}

// (abs(x1 - x2) + 1) * (abs(y1 -y2) + 1)
long area(Coord p, Coord q){
  long x = abs(p.x - q.x) +1;
  long y = abs(p.y - q.y) +1;

  return x*y;
}

Coord str_to_coord(char* str) {
  Coord c;
  int x = 0,y = 0;
  sscanf(str, "%d,%d",&x,&y);
  c.x = x;
  c.y = y;

  return c;
}