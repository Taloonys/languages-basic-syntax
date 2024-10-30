import kotlinx.coroutines.*

suspend fun suspFunc() {                    // `suspend` allow function to stop function if need and resume
    delay(2000)                             // .. could be used in `coroutines` or another `suspend funcs`
}


// `runBlocking` -> Runs a new coroutine in main thread and blocks main thread until its completion
fun main() = runBlocking {                  
    launch {                                // launch a top level coroutine
        delay(1000)                         // another way to `sleep`
        suspFunc()							// corotuine is suspended, but top-level coroutine is working
        println("Top-level coroutine")
    }
    println("It needs sth after coroutine")
}


suspend fun corScope() = coroutineScope {   // coroutines are scoped and sticked to that scope lifetime
    launch {
        delay(1000)
        println("scope, 1 coroutine")
    }

    launch {
        delay(1000)
        println("scope, 2 coroutine")
    }
}