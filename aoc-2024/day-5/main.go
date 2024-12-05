package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type instruction struct {
	num, val int64
}

func main() {
	readFile, _ := os.Open("inputs.txt")

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	var ints []instruction
	var sets [][]int64
	var correctSets [][]int64

	// read instructions
	for fileScanner.Scan() {
		if fileScanner.Text() == "" {
			break
		}
		ints = append(ints, getRules(fileScanner.Text()))
	}

	for fileScanner.Scan() {
		sets = append(sets, getSets(fileScanner.Text()))
	}

	// main loop
	for _, set := range sets {
		faulty := false
		for i := 0; i < len(set); i++ {
			rules := filter(ints, set[i])
			if len(rules) == 0 {
				continue
			}

			pre := set[0:i]
			_, idx := checkSlice(pre, rules)

			if idx != -1 {
				faulty = true
				set = fixSet(set, i, rules, ints)
			}
		}
		if faulty {
			fmt.Println("FAULTY ADDED: ", set, "\n")
			correctSets = append(correctSets, set)
		}
	}

	var sum int64 = 0
	for _, set := range correctSets {
		sum += set[len(set)/2]
	}

	fmt.Println("The sum is: ", sum)

	readFile.Close()
}

func fixSet(s []int64, i int, rules []instruction, allrules []instruction) []int64 {
	faulty := true
	for faulty {
		pre := s[0:i]
		_, idx := checkSlice(pre, rules)

		if idx == -1 {
			faulty = false
			break
		}

		temp := s[idx]
		s[idx] = s[i]
		s[i] = temp

		rules = filter(allrules, s[i])
	}

	return s
}

func getRules(l string) (ret instruction) {
	ln := strings.Split(l, "|")

	num, _ := strconv.ParseInt(ln[0], 10, 64)
	val, _ := strconv.ParseInt(ln[1], 10, 64)

	return instruction{num, val}
}

func checkSlice(s []int64, i []instruction) (int64, int) {
	for j := 0; j < len(s); j++ {
		for _, inst := range i {
			if s[j] == inst.val {
				return s[j], j
			}
		}
	}
	return -1, -1
}

func getSets(l string) (ret []int64) {
	ns := strings.Split(l, ",")
	for _, v := range ns {
		i, _ := strconv.ParseInt(v, 10, 64)
		ret = append(ret, i)
	}
	return
}

func filter(list []instruction, val int64) (ret []instruction) {
	for _, v := range list {
		if v.num == val {
			ret = append(ret, v)
		}
	}
	return
}
