package oop

type power = int

type dummyStruct struct {
	power power
	age   int
}

type inner struct {
	value int
}

type big struct {
	value int
	inner inner
}

type Node struct {
	value int
	// next  Node 	// <-- error, can't use itself
	next *Node // <-- frwd ptr fix
}

func StructInit() {
	var dummy0 dummyStruct
	dummy1 := dummyStruct{5, 23}
	dummy2 := dummyStruct{power: 10, age: 23}

	dummy0.power = max(dummy1.power, dummy2.power)
	dummy0.age = min(dummy1.age, dummy2.age)

	println(dummy0.power, dummy0.age)

	var big big = big{
		value: 10,
		inner: inner{
			value: 5,
		},
	}

	println(big.value, big.inner.value)
}
