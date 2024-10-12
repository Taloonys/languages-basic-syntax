package main

import (
	"fmt"
	"os"
)

func fileOpen() {
	var filename string = "sample.txt"

	if _, err := os.Create(filename); err != nil {
		panic("Couldn't create file")
	}

	if file, err := os.OpenFile(filename, os.O_WRONLY, 0666); err == nil {
		defer file.Close()

		println("Opened file")
	}
}

func terminalTreat() {
	var number int
	fmt.Print("Enter number")
	fmt.Scan(&number) // scan user input
}
