package oop

import "fmt"

/*
	In go you can stick method to a type "from outside", like a trait
*/

type Library /* = */ []string

func (obj Library) StickedPrint(stick_value int) {
	for _, element := range obj {
		fmt.Println("Value", element, " with a sticked value ", stick_value)
	}
}

func MethodStickUse() {
	var lib Library = Library{"Book1", "Book2"}
	lib.StickedPrint(666)
}

type BetterLibrary struct {
	store    []string
	quantity int
}

func (obj *BetterLibrary) appendBook(book string) { // <-- ptr allow to change object's members
	(*obj).store = append(obj.store, book)
}

func (obj BetterLibrary) size() int {
	return obj.quantity
}
