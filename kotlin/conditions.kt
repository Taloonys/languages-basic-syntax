fun if_block() {
    val a = 10

    if (1 !in 1..5) {
        return
    } else if (a in 2..3) {
        return
    } else {
        return
    }

    val b = if (1 in 1..5) { 
        println("a is 5") 
        5 
    } else {
        10
    }
}


fun when_block() {
    when (val b = a + 1) {                      // switch analog
        1,2         -> return
        !in 3..9    -> return
        2 + 9       -> println("a is $a")
        else        -> return
    }

    val isBetweenTwoAndTwenty = when (a) {
            in 3..19    -> true
            else        -> false 
    }
}


fun for_block() {
    for (i in 1..2) {
        continue
    }

    while (true) {
        break
    }

    do {
        break
    } while (true)

    toploop@ for (i in 1..2)    // loop marking
    {              
        for (j in 1..2) {
            break@toploop;      // mark usage
        }
    }
}