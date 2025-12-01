# Advent of Code 2025

```
                 *             *
        *              *                *
             *                               
      *           ADVENT OF CODE 2025         *
                                                  
         *      25 Days, 25 Languages      *
    *                                            *
             *                       *
         .        *         .             .
    *          *        *        *            *
        .    /\      *      /\       /\    .
            /  \          */  \  *  /  \
           /    \    *     /    \   /    \    *
      *   /  /\  \        /  /\  \ /  /\  \
         /  /  \  \   *  /  /  \  X  /  \  \  *
        /  /    \  \    /  /    \/ \/    \  \
       /__/      \__\  /__/     /\  \     \__\
    *     ||||          ||||   ||||  ||||       *
          ||||          ||||   ||||  ||||
```
![C](https://img.shields.io/badge/C-A8B9CC?style=flat-square&logo=c&logoColor=black)

A polyglot journey through Advent of Code 2025, solving each day's puzzle in a different programming language.

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