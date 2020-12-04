package main

import (
	"bufio"
	"os"
)

func main() {
	file, err := os.Open("day3\\input")
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	items := make([]string, 0)

	for scanner.Scan() {
		items = append(items, scanner.Text())
	}
	rightCounter := 3
	hit := 0
	for i := 1; i < len(items)-1; i++ {
		if items[i][rightCounter] == '#' {
			hit = hit + 1
		}
		rightCounter = rightCounter + 3
		if rightCounter > len(items[1]) {
			rightCounter = rightCounter - len(items[1])
		}
	}
	mt.Println(hit)


func rightIt(curr, iter, max int) int {
	curr = curr + iter
	if curr > max {
		urr = curr - max
	}
	eturn curr
}
