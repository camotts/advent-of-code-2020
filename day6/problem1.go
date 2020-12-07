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
	answers := make(map[rune]bool)
	totalAnswers := 0
	for scanner.Scan() {
		item := scanner.Text()
		if item == "" {
			totalAnswers = totalAnswers + len(answers)
			answers = make(map[rune]bool)
		}
		for _, char := range item {
			answers[char] = true
		}
	}
	totalAnswers = totalAnswers + len(answers)
	fmt.Println(totalAnswers)
}
