package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("day5\\input")
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	takenSeats := make([]int, 0)
	for scanner.Scan() {
		rows := makeRange(0, 127)
		seats := makeRange(0, 7)
		item := scanner.Text()
		for i, char := range item {
			if i <= 6 {
				if char == 'F' {
					rows = rows[0 : len(rows)/2]
				} else {
					rows = rows[len(rows)/2:]
				}
			}
			if i > 6 {
				if char == 'L' {
					seats = seats[0 : len(seats)/2]
				} else {
					seats = seats[(len(seats) / 2):]
				}
			}
			if i == len(item)-1 {
				id := rows[0]*8 + seats[0]
				takenSeats = append(takenSeats, id)
			}
		}
	}
	for i := 0; i < len(takenSeats) - 1; i++ {
		for j := 0; j < len(takenSeats) - i - 1; j++ {
			if (takenSeats[j] > takenSeats[j+1]) {
				temp := takenSeats[j]
				takenSeats[j] = takenSeats[j+1]
				takenSeats[j+1] = temp
			}
		}
	}
	for i := 0; i < len(takenSeats) - 2; i++ {
		if takenSeats[i] +1 != takenSeats[i+1] {
			fmt.Println("Found break!", takenSeats[i], takenSeats[i+1])
			break
		}
	}
}

func makeRange(min, max int) []int {
	a := make([]int, max-min+1)
	for i := range a {
		a[i] = min + i
	}
	return a
}
