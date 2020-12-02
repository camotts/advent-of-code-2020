package main

import (
	"bufio"
	"fmt"
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
	for scanner.Scan() {
		num, _ := strconv.Atoi(scanner.Text())
		items = append(items, num)
	}
	for i := 0; i < len(items); i++ {
		for j := i; j < len(items); j++ {
			for k := j; k < len(items); k++ {
				if items[i]+items[j]+items[k] == 2020 {
					fmt.Println(items[i] * items[j] * items[k])
				}
			}
		}
	}
}
