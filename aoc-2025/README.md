# Advent of Code 2025
```
        *                                                        *
   ___      _                 _            __   ____          _      
  / _ \    | |               | |          / _| / ___|        | |     
 / /_\ \ __| |_   _____ _ __ | |_    ___ | |_ | |     ___  __| | ___ 
 |  _  |/ ` \ \ / / _ \ '_ \| __|  / _ \|  _|| |    / _ \/ _` |/ _ \
 | | | | (_| |\ V /  __/ | | | |_  | (_) | |  | |___| (_) | (_| |  __/
 \_| |_/\__,_| \_/ \___|_| |_|\__|  \___/|_|   \____|\___/ \__,_|\___|
                                                                        
       *           * 25 Days, 25 Languages *               *          
  ^                     2025 Edition                           ^
 /o\         ^                                    ^           /o\
 /|\         |\                                  /|\          /|\
 /^\        /^\                                  /^\          /^\
  |          |                                    |            |
```
![C](https://img.shields.io/badge/c-%2300599C.svg?style=for-the-badge&logo=c&logoColor=white)

## 🔗 Quick Links
[Day 1 - C](./day-1) | [Day 2 - C](./day-2) | [Day 3 - C](./day-3)

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
