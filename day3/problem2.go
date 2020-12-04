package main

import (
	"bufio"
	"fmt"
	"os"
)

type combo struct {
	right int
	down int
}

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
	
	hit := 1
	groups := [...]combo{
		combo{
			right: 1,
			down: 1,
		},
		combo{
			right: 3,
			down: 1,
		},
		combo{
			right: 5,
			down: 1,
		},
		combo{
			right: 7,
			down: 1,
		},
		combo{
			right: 1,
			down: 2,
		},
	}
	for _, group := range groups {
		groupHits := 0
		rightCounter := group.right
		for i := group.down; i < len(items); i = i + group.down {
			if items[i][rightCounter] == '#' {
				groupHits = groupHits + 1
			}
			rightCounter = rightIt(rightCounter, group.right, len(items[i])-1)
		}
		hit = hit * groupHits
	}
	fmt.Println(hit)
}

func rightIt(curr, itter, max int) int {
	curr = curr + itter
	if curr > max {
		curr = curr - max - 1
	}
	return curr
}
