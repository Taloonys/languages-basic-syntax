class Dog(val name: String) {                   /**< It's a PRIMARY constructor*/

    var breed: String = ""                      /**< MUST be init'ed */
    lateinit var someValue: Int                 /**< `lateinit` allow be init'ed later */

    init {                                      /**< Constructor, `init = ` isn't allowed*/
        println("Dog name is $name")
    }

    constructor(name: String, breed: String)    /**< SECONDARY constructor */
        : this(name)                            /**< MUST call PRIMARY explicitly if has input arg */
    {
        println("*secondary constructor*")
        this.breed = breed
        lateinit = 5
    }
}


fun main() {
    val dog = Dog()
}