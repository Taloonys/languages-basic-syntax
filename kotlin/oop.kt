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


//
// Classes
//

class Dog(val name: String) {                   /**< Pass along with class init */

    var breed: String = ""                      /**< MUST be init'ed */

    init {                                      /**< Constructor, `init = ` isn't allowed*/
        println("Dog name is $name")
    }

    constructor(name: String, breed: String)    /**< Secondary constructor */
        : this(name)
    {
        println("*secondary constructor*")
        this.breed = breed
    }
}


open class Creature {                           /**< Base class */
    val color: String = "white"                 /**< will be inherited*/

    fun eat() = println("creature eat")
}

class Cat(val name: String) : Creature() {

    // `color` is inherited

    fun sound() = println("meow")
}