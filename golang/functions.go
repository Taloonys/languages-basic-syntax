package main

func Increment(x int, y int) { // they are passed BY VALUE!
	x++
	y++
	return
}

func PointerPass(x *int) { // in-out (pointer) pass
	(*x)++
}

func VarArgUse(numbers ...int) {
	for _, number := range numbers {
		number--
	}
}

func SlicesInputExample() {
	VarArgUse([]int{1, 2, 3}...)
}

// this func-type is -> func(int, int, string, string, string) string
func WeirdInput(x_int, y_int int, str1, str2, str3 string) string { return str2 }

func Summ(x int, y int) int {
	return x + y
}

func NamedReturn(x int, y int) (z int) { // z would be returned
	z = Summ(x, y)
	return
}

func MultiReturn(x int, y int) (int, int) {
	return x, y
}

func UseFuncPointers() {
	var tmp_func func(int, int) int = NamedReturn
	/*
		// not possible
		var tmp_func1 func(int, int) int, int = MultiReturn
	*/

	tmp_func(1, 2)
}

func AnonymosFuncUsage() {
	summ_func := func(x, y int) int { return x + y }
	summ_func(1, 2)

	var multiplier = 2
	multiply := func(x int) int {
		return x * multiplier // <-- can capture local variables
	}
	multiply(50)
}

// FEATURE
func DeferUsage() {
	finish_call := func(x int) { println("Defer Call - ", x) }
	defer finish_call(1) // <-- !!!
	defer finish_call(2) // <-- !!!

	println("DeferUsage func body")
	/*
		In the end of the function would calls:
		- finish_call(2)
		- finish_call(1)
		Defers call from bottom to top!
	*/
}
