package main

func PointersInit() {
	var x int = 5
	var px *int = &x
	println(*px)

	var empty_ptr *int // <-- it is nil
	if empty_ptr != nil {
		panic("empty_ptr somehow broke go-laws")
	}
}

func NewUsage() {
	// there is a garbage collector in language
	var px *int = new(int)
	*px = 10
}
