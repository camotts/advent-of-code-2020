package main

import (
	"fmt"
	"bufio"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("day1\\input1")
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	items := make([]int, 0)
	started := false
	currNum := -1
    for scanner.Scan() {
		num, _ := strconv.Atoi(scanner.Text())
		items = append(items, num)
		if !started {
			started = true
			currNum = num
		} else {
			if currNum + num == 2020 {
				fmt.Println(currNum*num)
			}
		}
	}
	for i := 1; i < len(items); i++ {
		for j := i; j < len(items); j++ {
			if items[i] + items[j] == 2020 {
				fmt.Println(items[i] * items[j])
			}
		}
	}
}