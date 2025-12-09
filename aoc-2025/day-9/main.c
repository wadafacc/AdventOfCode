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
// calculate area between two points
long area(Coord p, Coord q);
// raycast: returns 1 if inside, 0 if outside
int is_inside(int** arr, int x, int y);
// checks all tiles of rectangle if they're inside polygon
int check_bounds(int** arr, Coord q, Coord p, int offset_x, int offset_y);

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

    valid_tiles[coords[i].y - min_y][coords[i].x - min_x] = VALID;
    
    // fill in edges
    int x1 = coords[i].x, x2 = coords[next_idx].x;
    int y1 = coords[i].y, y2 = coords[next_idx].y;
    
    for (int dy = MIN(y1,y2); dy <= MAX(y1,y2); dy++) {
      for (int dx = MIN(x1,x2); dx <= MAX(x1,x2); dx++) {
        valid_tiles[dy - min_y][dx - min_x] = VALID;
      }
    }
  }

  // for (int i = 0; i < h; i++) {
  //   for (int j = 0; j < w; j++) {
  //     if (valid_tiles[i][j] == 2) {
  //       printf("@");
  //       continue;
  //     } 
  //     printf("%c", valid_tiles[i][j] == 1 ? '#' : '.');
  //   }
  //   printf("\n");
  // }

  long biggest_a = 0;
  for (int i = 0; i < line_count; i++) {
    for (int j = i+1; j < line_count; j++) {
      // printf("Checking: (%d %d) - (%d %d)\n", coords[i].x, coords[i].y, coords[j].x, coords[j].y);
      if (check_bounds(valid_tiles, coords[i], coords[j], min_x, min_y) == 0) continue;
      long c_area = area(coords[i], coords[j]);
      // printf("-> valid (%ld)\n", c_area);
      if (c_area > biggest_a) biggest_a = c_area;
    }
  }

  return biggest_a;
}

// draw line from above to corner
int is_inside(int** arr, int x, int y) {
  if (arr[y][x] == VALID) return 1;

  int crossings = 0;
  int prev = 0;  // Previous tile state
  
  for (int dy = 0; dy < y; dy++) {
    int curr = (arr[dy][x] == VALID) ? 1 : 0;
    
    // Count transition from non-boundary to boundary
    if (curr == 1 && prev == 0) crossings++;
    prev = curr;
  }
  // printf("Count: %d\n", crossings);
  return crossings % 2 == 0 ? 0 : 1;
}

// returns 1 if all points are inside, 0 if any is outside polygon
int check_bounds(int** arr, Coord q, Coord p, int offset_x, int offset_y) {
  int min_x = MIN(q.x, p.x), max_x = MAX(q.x, p.x);
  int min_y = MIN(q.y, p.y), max_y = MAX(q.y, p.y);

  for (int y = min_y; y <= max_y; y++) {
    for (int x = min_x; x <= max_x; x++) {
      if (is_inside(arr, x - offset_x,y - offset_y) != 1) return 0;
    } 
  }
  return 1;
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