/*
 * It's not an object of class, it's a different class category
 * And it's also like a singleton
 * */

interface Boxer {
    fun pack()             

    var id: Int
}

object MyBox : Boxer {
    override fun pack()     = println("pack")       /**< like a static member */
    override var id: Int    = 555                   /**< like a static member */
}


class Dummy {
    companion object {
        val static_value: Int = 13                  /**< like a static member */
    }
}


fun main() {
    MyBox.pack()
    MyBox.id = 9999

    val person = object {                           // anonymos class
        val name = "Johny"

        fun work() = println("idk, man")
    }
    person.work()

    val dmy = Dummy.static_value                    // companion object use
}
