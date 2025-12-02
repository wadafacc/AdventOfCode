import time

def main():
    start_time = time.time()

    ranges = []
    part1, part2 = 0, 0

    with open("day02.txt", "rt") as fin:
        for pair in fin.readline().split(","):
            ranges.append(tuple(map(int, pair.split("-"))))

    for first, last in ranges:
        for current in range(first, last + 1):
            s = str(current)
            if s in (s + s)[1:-1]:
                if s[: len(s) // 2] == s[len(s) // 2 :]:
                    part1 += current
                part2 += current

    end_time = time.time()

    print(f"Part 1: {part1}\nPart 2: {part2}")
    print(f"Execution time: {end_time - start_time:.4f} seconds")  # ~0.27 seconds

if __name__ == "__main__":
    main()