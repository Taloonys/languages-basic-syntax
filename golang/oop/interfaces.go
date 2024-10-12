package oop

type creature interface {
	breath()
	jump()
}

func jump(iobj creature) {
	iobj.jump()
}

/*
	In GO struct could bind to interfaces implicitly if they defines interfaces's methods
*/

type dog struct{}

/*^*/
func (obj dog) breath() {
	print()
}

/*^*/
func (obj dog) jump() {
	print()
}

func usage() {
	// c := creature{}	// err

	var dogo creature = dog{}
	dogo.breath()

	dogo2 := dog{}
	jump(dogo2) // <-- it satisifies interface functions, so we can use it
}
