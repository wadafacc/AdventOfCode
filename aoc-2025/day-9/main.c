#include<stdio.h>
#include<stdlib.h>
#include<util.h>

const char* filename = "./day-9/input.txt";

typedef struct Coord {int x,y; } Coord;

Coord str_to_coord(char* str);
long area(Coord p, Coord q);

int main() {
  int line_count = 0, line_len = 0;
  char** lines = file_lines(filename, &line_count, &line_len);
  Coord* coords = malloc(line_count * sizeof(Coord));
  for (int i = 0; i < line_count; i++) {
    coords[i] = str_to_coord(lines[i]);
  }

  long biggest_a = 0;
  for (int i = 0; i < line_count; i++) {
    for (int j = i+1; j < line_count; j++) {
      long c_area = area(coords[i], coords[j]);
      if (c_area > biggest_a) biggest_a = c_area;
    }
  }
  printf("%ld\n", biggest_a);
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