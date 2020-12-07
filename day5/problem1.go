package main

import (
	"bufio"
	"os"
	"fmt"
)

func main() {
	file, err := os.Open("day5\\input")
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	highestSeat := 0
	for scanner.Scan() {
		rows := makeRange(0, 127)
		seats := makeRange(0,7)
		item := scanner.Text()
		for i, char := range item {
			if i <= 6 {
				if char == 'F' {
					rows = rows[0:len(rows)/2]
				} else {
					rows = rows[len(rows)/2:]
				}
			}
			if i > 6 {
				if char =='L' {
					seats = seats[0:len(seats)/2]
				} else{
					seats = seats[(len(seats)/2):]
				}
			}
			if i == len(item) - 1{
				id := rows[0] * 8 + seats[0]
				if id > highestSeat {
					highestSeat = id
				}
			}
		}
	}
	fmt.Println(highestSeat)
}

func makeRange(min, max int) []int {
    a := make([]int, max-min+1)
    for i := range a {
        a[i] = min + i
    }
    return a
}