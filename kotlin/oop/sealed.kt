/*
 * Well, it's not like a `final` keyword
 * */

 sealed class Shape() {
    data class Circle(var r: Int)                       : Shape()

    class Square (var a: Int)                           : Shape()

    object Abstraction						            : Shape()

    sealed class Line(var len: Int)                     : Shape()
 }

 class Triangle(var a: Int, var b: Int, var c: Int)     : Shape()


 fun main() {
    val circle  = Shape.Circle(5)
    val a       = Shape.Abstraction
    if (a is Shape.Abstraction) { ; }
 }