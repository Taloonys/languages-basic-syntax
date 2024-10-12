package main

func Conditions() {
	var a, b = 1, 2
	var c bool = a != b

	if c {
		DoNothing()
	}

	if tmp := 10; a < tmp {
		DoNothing()
	} else {
		return
	}

	// There is no "if..else if..."
}

func Switching() {
	var a int = 100

	switch a {
	case 1:
		DoNothing()
	case 2:
		DoNothing()
	case 5, 6, 111:
		DoNothing()
	default:
		DoNothing()
	}
}

func Cycling() {
	for i := 1; i < 10; i++ {
		if i == 2 {
			continue
		}

		if i == 5 {
			break
		}
	}

	/*
		for ;;{
		}
		.. is possible
	*/

	ArrayParsing()
}

func ArrayParsing() {
	numbers := [3]string{0: "zero", 1: "one", 2: "two"}
	for index, value := range numbers {
		println(index, value)
	}
}
