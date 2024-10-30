fun main() {
    try {
        val n = 2 / 0
        // throw Exception("any text")
    } catch (e: Exception) {
        println("""
                ${e.message}
                ${e.stackTrace}
                """)
    } finally {
        println("whatever...")
    }

}