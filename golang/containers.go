package main

// ----------------- Slices -----------------
func SliceInit() {
	var slice []int // <---- like arrays, but there is no size
	var slice1 = []string{"Tasd", "asd"}
	slice2 := []int{1, 2, 3}

	var users []string = make([]string, 2)
	users[0] = "User0"
	users[1] = "User1"

	println(slice, slice1, slice2)
}

func SliceOperations() {
	slice := []int{1, 2, 3}

	slice = append(slice, 4)
	sliced_slice := slice[2:4] // got elements in range [2, 4)
	sliced_slice = slice[:2]   // got elemnts in range [0, 2]
	sliced_slice = slice[2:]   // got elemtns ing range [2, end()]

	println(sliced_slice)

	// no remove or erase operations -_- !
}

// ----------------- Views -----------------

func ViewInit() {
	var users = map[string]int{ // <-- [key]value -- based on hash table
		"ChocoBoy":  1,
		"CringeMan": 2,
		"Woman":     3,
	}

	numbers := make(map[int]string)

	println(users, numbers)
}

func ViewOperations() {
	numbers := make(map[int]string)
	numbers[0] = "zero" // <-- append
	delete(numbers, 0)  // <-- remove
}
