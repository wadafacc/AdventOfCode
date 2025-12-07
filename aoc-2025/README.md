# Advent of Code 2025
```
        *                                                        *
   ___      _                 _            __   ____          _      
  / _ \    | |               | |          / _| / ___|        | |     
 / /_\ \ __| |_   _____ _ __ | |_    ___ | |_ | |     ___  __| | ___ 
 |  _  |/ _` \ \ / / _ \ '_ \| __|  / _ \|  _|| |    / _ \/ _` |/ _ \
 | | | | (_| |\ V /  __/ | | | |_  | (_) | |  | |___| (_) | (_| |  __/
 \_| |_/\__,_| \_/ \___|_| |_|\__|  \___/|_|   \____|\___/ \__,_|\___|
                                                                        
       *           * 25 Days, All in C     *               *          
  ^                     2025 Edition                           ^
 /o\         ^                                    ^           /o\
 /|\        /|\                                  /|\          /|\
 /^\        /^\                                  /^\          /^\
  |          |                                    |            |
```
![C](https://img.shields.io/badge/c-%2300599C.svg?style=for-the-badge&logo=c&logoColor=white)

## 🔗 Quick Links
[Day 1 - C](./day-1) | [Day 2 - C](./day-2) | [Day 3 - C](./day-3) | [Day 4 - C](./day-4) | [Day 5 - C](./day-5) | [Day 6 - C](./day-6) | [Day 7 - C](./day-7)

## 🎄 Progress

### Day 1: North Pole Navigation Simulator ⭐⭐
**Language:** C  
**Challenge:** Track position on a circular track (0-99) starting at position 50. Count how many times position 0 is crossed while following movement instructions.

**Key Concepts:**
- Modular arithmetic for circular tracks
- Position tracking with wrapping
- Parsing directional commands (L/R + steps)

**Installation & Running:**
```bash
cd day-1
gcc -o solution solution.c
./solution
```

**Input Format:**
```
L 15
R 8
L 42
```

---

### Day 2: Gift Shop ⭐⭐
**Language:** C  
**Challenge:** Identify invalid product IDs in ranges. Invalid IDs are numbers formed by repeating a digit sequence.

**Part 1:** Find IDs that are a sequence repeated exactly twice  
Answer: `24747430309`

**Part 2:** Find IDs that are a sequence repeated at least twice  
Answer: `30962646823`

**Key Concepts:**
- String manipulation and pattern matching
- Sliding window algorithm
- Range iteration and validation
- Custom string utilities (split, compare, repeat)

**Installation & Running:**
```bash
cd day-2
gcc -o solution solution.c -lm
./solution
```

**Input Format:**
```
11-22,95-115,998-1012,1188511880-1188511890,...
```

---

### Day 3: Lobby ⭐⭐
**Language:** C  
**Challenge:** Power an escalator by selecting batteries from banks to maximize joltage output.

**Part 1:** Select exactly 2 batteries per bank to form the largest 2-digit number  
Answer: `17359`

**Part 2:** Select exactly 12 batteries per bank to form the largest 12-digit number  
Answer: `172787336861064`

**Key Concepts:**
- Greedy algorithm for digit selection
- Maintaining left-to-right order constraint
- Large number handling with `long`
- Custom power function for place value calculation

**Installation & Running:**
```bash
cd day-3
gcc -o solution solution.c
./solution
```

**Input Format:**
```
987654321111111
811111111111119
234234234234278
```

---

### Day 4: Printing Department ⭐⭐
**Language:** C  
**Challenge:** Optimize forklift work by identifying accessible paper rolls in a grid. A roll is accessible if it has fewer than 4 adjacent rolls.

**Part 1:** Count initially accessible rolls  
Answer: `1578`

**Part 2:** Simulate iterative removal - keep removing accessible rolls until none remain  
Answer: `10132`

**Key Concepts:**
- 2D grid traversal and neighbor checking
- Cellular automaton-like state evolution
- Dynamic memory allocation for grid storage
- Iterative simulation with termination condition
- Boundary checking for 8-directional adjacency

**Installation & Running:**
```bash
cd day-4
gcc -o solution solution.c
./solution
```

**Input Format:**
```
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
```

**Algorithm:**
1. Load grid and identify all paper rolls (`@`)
2. For each roll, count adjacent rolls (8 directions)
3. Mark rolls with < 4 neighbors as accessible (`x`)
4. Remove accessible rolls and repeat until no more can be removed
5. Sum total removals across all iterations

---

### Day 5: Cafeteria ⭐⭐
**Language:** C  
**Challenge:** Process ingredient database to determine which IDs are fresh based on range specifications.

**Part 1:** Count how many available ingredient IDs fall within fresh ranges  
Answer: `868`

**Part 2:** Calculate total count of all IDs considered fresh by the ranges  
Answer: `354143734113772`

**Key Concepts:**
- Range merging and overlap detection
- Interval consolidation algorithm
- Dynamic memory management for range arrays
- Sorting ranges for efficient merging
- Range comparison logic (overlap/disjoint detection)

**Installation & Running:**
```bash
cd day-5
gcc -o solution solution.c
./solution
```

**Input Format:**
```
3-5
10-14
16-20
12-18

1
5
8
11
17
32
```

**Algorithm:**
1. Parse fresh ingredient ID ranges from input
2. Sort ranges by starting position
3. Merge overlapping or adjacent ranges iteratively
4. For Part 1: Check which available IDs fall in merged ranges
5. For Part 2: Sum the sizes of all merged ranges (end - start + 1)

---

### Day 6: Trash Compactor ⭐⭐
**Language:** C  
**Challenge:** Help a young cephalopod with her math homework by solving vertical column-based arithmetic problems.

**Part 1:** Read problems left-to-right, where numbers are arranged vertically in columns  
Answer: `5346286649122`

**Part 2:** Read problems right-to-left (cephalopod style), building numbers column by column  
Answer: `10389131401929`

**Key Concepts:**
- 2D text parsing and column extraction
- Vertical number reading with digit position awareness
- Dynamic string-to-number conversion
- Bidirectional iteration (left-to-right vs right-to-left)
- Operation detection and accumulation

**Installation & Running:**
```bash
cd day-6
gcc -o solution solution.c -lutil
./solution
```

**Input Format:**
```
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
```

**Algorithm:**
1. Read all lines into memory, track longest line length
2. Iterate through each column position
3. Extract vertical digits from all rows except the last
4. Last row contains operation symbol (`*` or `+`)
5. Convert column characters to numbers, apply operation
6. Empty columns (all spaces) separate problems - add result to total
7. For Part 2: Process columns from right to left instead

---

### Day 7: Laboratories ⭐⭐
**Language:** C  
**Challenge:** Simulate tachyon beam behavior through a manifold with splitters. Count beam splits and quantum timeline branches.

**Part 1:** Count total number of times beams split in the manifold  
Answer: `1555`

**Part 2:** Count unique timelines in quantum many-worlds interpretation  
Answer: `12895232295789`

**Key Concepts:**
- Recursive path counting with memoization
- Dynamic programming for exponential branching
- 2D grid traversal and beam simulation
- Coordinate-based memoization to avoid recomputation
- Quantum mechanics interpretation (many-worlds)

**Installation & Running:**
```bash
cd day-7
gcc -o solution solution.c -lutil
./solution
```

**Input Format:**
```
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
```

**Algorithm:**
1. Parse manifold diagram into 2D grid
2. Simulate beam propagation downward from start position (`S`)
3. When beam hits splitter (`^`), create two new beams (left and right)
4. Beams pass through empty space (`.`) unaffected
5. For Part 1: Count total split events during simulation
6. For Part 2: Use memoization to count unique paths (timelines)
   - Each cell `(row, col)` caches number of paths from that point
   - Prevents exponential time complexity (2^70 → manageable)
7. Sum all possible timeline branches to exit

**Performance Note:**
Without memoization, Part 2 would require 2^70+ recursive calls (longer than the age of the universe). Memoization reduces this to O(rows × cols) complexity.

---

## 📝 Notes
- Each day's solution is contained in its own directory (`day-1/`, `day-2/`, etc.)
- Input files are stored as `input.txt` within each day's directory
- Solutions prioritize clarity and learning over optimization
- Some solutions require the math library: compile with `-lm` flag

## 🎯 Goals
- Explore 25 different programming languages
- Complete all Advent of Code 2025 challenges
- Learn new paradigms and approaches to problem-solving

---

*Happy Coding! 🎅*