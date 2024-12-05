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
		rule := getRules(fileScanner.Text())

		ints = append(ints, rule)
	}

	for fileScanner.Scan() {
		ns := strings.Split(fileScanner.Text(), ",")
		var temp []int64
		for _, v := range ns {
			i, _ := strconv.ParseInt(v, 10, 64)
			temp = append(temp, i)
		}
		sets = append(sets, temp)
	}

	for _, set := range sets {
		faulty := false
		for i := 0; i < len(set); i++ {
			rules := filter(ints, set[i])
			if len(rules) == 0 {
				continue
			}
			pre := set[0:i]
			if checkSlice(pre, rules) {
				faulty = true
			}
		}
		if !faulty {
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

func getRules(l string) (ret instruction) {

	ln := strings.Split(l, "|")

	num, _ := strconv.ParseInt(ln[0], 10, 64)
	val, _ := strconv.ParseInt(ln[1], 10, 64)

	return instruction{num, val}
}

func filter(list []instruction, val int64) (ret []instruction) {
	for _, v := range list {
		if v.num == val {
			ret = append(ret, v)
		}
	}

	return
}

func checkSlice(s []int64, i []instruction) bool {
	for _, v := range s {
		for _, inst := range i {
			if v == inst.val {
				return true
			}
		}
	}
	return false
}
