data class Context(var title: String, var id: Int)      // just hold data

fun main() {
	var c1 = Context("Conecpt", 1)
    val c2 = Context("Category", 2)
    
    println(c1.toString())                      // just print data class members
    println(c1 == c2)                           // could be compared
    
    val c3 = c2.copy(title = "NewCategory")     // copy + edited value
    println(c3)
}