package main

// there are no exceptions tbh, only panic()

func PanicUsage() {
	var a int = 5
	if a != 10 {
		panic("a is not 10") // <-- throw analog
	}
}
