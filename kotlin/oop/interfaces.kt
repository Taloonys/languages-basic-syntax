/*
 * All interface's properties are open and overridable
 * */

interface MyMonad {
    fun bind()                                  // direved must implement this
    fun map() { /* Empty */ }
    fun pack() = println("Monad pack")
}

interface MyBox {
    fun pack() = println("Box pack")
    fun unpack() {}
}


class ValueDummy : MyMonad, MyBox {
    override fun bind() { /* Empty */ }
    override fun pack() {                       // has to override, cuz 2 packs in differenc interfaces
        super<MyMonad>.pack()                   // way to choose
        super<MyBox>.pack()
    }
    
    fun exec() {
        super.unpack()
        super.map()
    }
}


fun main() {
    val vl = ValueDummy()
    vl.pack()
    vl.bind()
}