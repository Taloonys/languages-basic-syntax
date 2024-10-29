fun Dummy.anotherWork() = println("another work")               // extension

class Dummy {
    fun work() = println("work")    
}

fun Dummy.anotherGreatWork() = println("another great work")    // extension


fun extension_function() {
    var dmy = Dummy()

    dmy.work()
    dmy.anotherWork()
    dmy.anotherGreatWork()
}


fun Int.summWith(x: Int, y: Int): Int = this + x + y            // extended based type

infix fun Int.mutiplyWith(x: Int): Int = this * x               // infix extension

fun base_type_extension() {
    val tmp = 5.summWith(10, 100)
    val tm  = 5 mutiplyWith 10                                  // infix usage
    println("""
            tmp = $tmp
            tm  = $tm
            """)
}