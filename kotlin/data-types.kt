/*
 * Every single type is a class, every single "instance" is an object, even primitive objects
 */


fun paramsInit() {
    var num1: Int
    num1 = 1

    var num2: Int = 2

    val num3 = 3                    // auto dedact
    var num4 = 4

    val doub1       = 2.0
    //val num5: Int   = doub1       // err, no implicit cast
    val num5: Int   = doub1.toInt()

    val any1: Any   = "NowStr"

}


fun ranges() {
    val r5 = 'a'..'z'
    val r1 = 1..10
    val r2 = 10 downTo 1
    val r3 = 10 downTo 2 step 2
    val r4 = 10.downTo(1)
    val r6 = 1.rangeTo(1)
}


const val num2 = 2          /**< compile time constant & only for non-func/non-class scope */

fun constant_value() {
    val num1: Int           // could be assigned only once, so `val` -> only init
    num1 = 11
    //num1 = 55             // err
}


val pi: Float by lazy {     // apply lazy evaluation
    3.14f
}

fun main() {
    var arr = Array<Int>(2) {1, 2}  

    val tmp = 2 * pi
}