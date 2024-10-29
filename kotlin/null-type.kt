fun main() {
    val textFill: String? = "text"
    val textNull: String? = null

    textNull?.let { println (textNull) }    /**< like a safe call of `null` */
    println("$textNull")                    // usually also ok

    val len = textNull?.length ?: -1        // `?:` operator -> if can't evaluate length -> return -1
    println(len)                            

    println("$textNull!!.length")           // throws exception if textNull is `null`
}