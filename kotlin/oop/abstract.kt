abstract class Person {
    abstract var name: String       // `abstract` also implies `open`
    abstract fun exist()

    open fun doSth() {}
    fun work() {}
}


class Josh : Person() {
    override var name = "Josh"
    override fun exist() = println("Don't want to")
}


fun main() {
    //val p       = Person()          //obvious error
    val josh    = Josh().exist()
}