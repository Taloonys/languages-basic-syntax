fun default_func(num: Int, str: String, bl: Boolean = false) 
{ /* Empty */ }


fun short_sum(x: Int, y: Int): Int = x + y;


fun named_params(): Int {
    return short_sum(y = 2, x = 10)
}


tailrec fun factorial(n: Int, acc: Int = 1): Int {      // tail recursion built-in
    return when {
        n == 0	-> acc
        else   	-> factorial(n-1, acc * n)
    }
}