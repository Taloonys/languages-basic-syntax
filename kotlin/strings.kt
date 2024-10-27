fun stringUsage() {
    val text: String        = "text"
    val long_text: String   = """
                              Huge
                              Text
                              """
}


fun string_interpolation() {
    val num1: Int           = 555555
    val num2: Int           = 999999
    val paramText: String       = "Numbers are: $num1, $num2"           //string interpolation
    val paramText1: String      = "Numbers are: ${num1}, ${num2}"
}