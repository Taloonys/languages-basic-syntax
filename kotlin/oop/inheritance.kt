open class Creature (_name: String = "") {          /**< Base class */
    val name = _name

    init {
        println("creature $name exist")
    }

	open fun eat() = println("creature $name eat")  /**< 'open' allows to be overriden */

    open val color: String = "white"            	/**< will be inherited*/

    val _id = 5                                 	/**< won't be overriden */
}


class Cat(_name: String = "")                       
	: Creature(_name) {                         	/**< also calling parent's PRIMARY constuctor */

    constructor(_name: String, _color: String) 
        : this(_name)                              /**< parent's secondary constructor call */
    {  
        this.color = _color
        println("cat exist")
    }

    fun sound() = println("$color cat meow")

    override fun eat() {
        super<Creature>.eat()                   	/**< parent's method call */
        println("cat eat")
    }

    override var color = "brown"                	/**< inherited and overrided */
}


fun main() {
    val cat  = Cat("Bobby").sound()
    val cat1 = Cat("Bobby", "Purple").sound()
}