/*
 * scope functions: 
 * - let
 * - run
 * - with
 * - apply
 * - also
 * */

class Dummy {
    var id      = 5
    var text    = "text"

    fun do() = println("do")
}

fun main() {
    var dmy = Dummy()

    Dummy().let {                   // like a small scope, but instance is accessed via `it`
        it.id = 55
        it.do()
        println(it.id)
    }

    with(dmy) {                     // requires object
        id      = 50
        text    = "heeeeeeeeeellooo"
        println("with is accessed to class via $this")
    }
    println(dmy.text)

    dmy.apply {                     // like `with`, but requires instance
        id      = 555
        text    = "reeeeeeeeee"
    }
    println(dmy.id)

    val dmy_result = dmy.run {      // run a "context"
        id      = 9
        job()
    }

    val dmy_also = Dummy()
        .also{                      // fluent inteface, but not interface...
            println("also 1") 
        }
        .also{
            println("also 2")
        }
        .job()

}