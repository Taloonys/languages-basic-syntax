//
// Params and types
//

package main

type color1 string   // <-- color1-type is like string, it's NAMED TYPE
type color2 = string // <-- the same, but this is NAME ALIAS

func ParamsDeclDef() {
	var hello string
	hello = "Hello Go .... !"

	var num = 555
	var value1, value2 = 5, 10

	instant_def_init := "Walrus"

	println(hello, num, value1, value2, instant_def_init)
}

func ParamsPacks() {

	var (
		name     = "Mike"
		age  int = 15
	)

	println(name, age)
}

func ConstParams() {
	const const_int = 555
	const constant_value string = "constant value"

	/*example*/
	// var int_num = 5
	// const const_int = int_num // error -> int_num is not constant

	const ( // <--- enum like
		one   = 1
		two   // 2
		three // 3
		four  = 4
		five  // 5
	)
}

func Operations() {
	var (
		a = 1
		b = 5
		c = 10
	)

	var t = a / b
	var tm = c % b

	t++
	tm--
}

func Arrays() {
	var numbers1 [5]int = [5]int{0, 1, 2, 3, 4}
	var numbers2 [5]int = [5]int{2, 2} // 2 2 0 0 0
	numbers3 := [5]int{1, 2, 3, 4, 5}
	var numbers4 = [...]int{1, 1, 1, 1, 1} // size evaluated = 5

	numbers4[0] = 999

	dictionary_like := [3]string{2: "red", 0: "Tom", 1: "Yepee"}
	println(len(numbers1), len(numbers2), len(numbers3), len(dictionary_like))
}
