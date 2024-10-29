fun sum(x: Int, y: Int): Int = x + y

fun sum(x: Int, y: Int, handler: (Int) -> Unit): Int {                  // in `()` are input types
    val ret = x + y
    handler(ret)
    return ret
}


fun reverse(str: String, myFunc: (String) -> String) {
    val result = myFunc(str)
    println(result)
}


fun main() {
    val s1 = sum(1, 2)

    val myLambda: (Int) -> Unit = { value: Int -> println(value) }      // `{ input,input -> body }`
    val s2 = sum(1, 2, myLambda)    

    var save_return_val: Int = 0
    val s3 = sum(1, 2, { ret: Int -> save_return_val = ret })
    println(save_return_val)

    reverse("abcdef", { it.reversed() })        /**< keyword `it`, that substitute input argument */
}