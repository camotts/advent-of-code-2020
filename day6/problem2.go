package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("day6\\input")
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	answers := make([]map[rune]bool, 0)
	totalAnswers := 0
	currPerson := 0
	personWithMostLetters := 0
	personLength := 0
	for scanner.Scan() {
		item := scanner.Text()
		if item == "" {
			totalAnswers = totalAnswers + countAnswers(answers, personWithMostLetters)
			answers = make([]map[rune]bool, 0)
			currPerson = 0
			personWithMostLetters = 0
			personLength = 0
			continue
		}
		if len(item) > personLength {
			personLength = len(item)
			personWithMostLetters = currPerson
		}
		for _, char := range item {
			if len(answers)-1 < currPerson {
				answers = append(answers, make(map[rune]bool))
			}
			answers[currPerson][char] = true
		}
		currPerson = currPerson + 1
	}
	totalAnswers = totalAnswers + countAnswers(answers, personWithMostLetters)
	fmt.Println(totalAnswers)
}

func countAnswers(answers []map[rune]bool, start int) int {
	total := 0
	if len(answers) == 1 {
		total = len(answers[0])
	} else {
		for k := range answers[start] {
			missing := false
			for i, other := range answers {
				if i == start {
					continue
				}
				if ok := other[k]; !ok {
					missing = true
					break
				}
			}
			if !missing {
				total = total + 1
			}
		}
	}
	return total
}
