# Advent of Code 2025

![C](https://img.shields.io/badge/C-A8B9CC?style=flat-square&logo=c&logoColor=black)

```
        *                                                        *
   ___      _                 _            __   ____          _      
  / _ \    | |               | |          / _| / ___|        | |     
 / /_\ \ __| |_   _____ _ __ | |_    ___ | |_ | |     ___  __| | ___ 
 |  _  |/ _` \ \ / / _ \ '_ \| __|  / _ \|  _|| |    / _ \/ _` |/ _ \
 | | | | (_| |\ V /  __/ | | | |_  | (_) | |  | |___| (_) | (_| |  __/
 \_| |_/\__,_| \_/ \___|_| |_|\__|  \___/|_|   \____|\___/ \__,_|\___|
                                                                        
       *           * 25 Days, 25 Languages *               *          
  ^                     2025 Edition                           ^
 /o\         ^                                    ^           /o\
 /|\         |\                                  /|\          /|\
 /^\        /^\                                  /^\          /^\
  |          |                                    |            |
```

A polyglot journey through Advent of Code 2025, solving each day's puzzle in a different programming language.

## 🔗 Quick Links

[Day 1 - C](#day-1-c)

## 🎄 Progress

### Day 1: C
**Language:** C  
**Challenge:** North Pole Navigation Simulator

The elves need help tracking their position on a circular track numbered 0-99. Starting at position 50, they receive a series of movement instructions:
- `L` followed by a number: move left (decrease position) that many steps
- `R` followed by a number: move right (increase position) that many steps

The track wraps around (position 0 connects to position 99). The goal is to count how many times they pass through position 0 during their journey.

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
...
```

Each line contains a direction (`L` or `R`) followed by a space and the number of steps.

---

## 📝 Notes

- Each day's solution is contained in its own directory (`day-1/`, `day-2/`, etc.)
- Input files are stored as `input.txt` within each day's directory
- Solutions prioritize clarity and learning over optimization

## 🎯 Goals

- Explore 25 different programming languages
- Complete all Advent of Code 2025 challenges
- Learn new paradigms and approaches to problem-solving

---

*Happy Coding! 🎅*