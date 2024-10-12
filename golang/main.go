package main

// import "fmt" 	// <-- single import
import (
	"fmt"
	"math"
)

/*
	link for language standards
	https://github.com/golang-standards/project-layout
*/

/*
	For packaging import stuff is:
	- vendor feature
	- go.mod
*/

/*
	If first letter of any instance is UpperCase, it's a Public one
	if it starts with lowercase, it's a private one (for package!)
*/

func DoNothing() {}

func main() {
	/* LongComment */
	// Short comment
}

func mathWarningSuppress() {
	math.Abs(-1)
}

func fmtWarningSuppress() {
	fmt.Print("...")
}
