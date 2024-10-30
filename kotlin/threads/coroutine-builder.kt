import kotlinx.coroutines.*

fun main() = runBlocking {

    var myCoroutine: Job = launch {
        println("myCoroutine")
        delay(1000)
    }

    myCoroutine.join()                  // keep main thread until `myCoroutine` finish
}

fun cooperativeCoroutine() = runBlocking {

    val job: Job = launch {
        println("coopCoroutine")
        delay(1000)
    }

    job.cancelAndJoin()                  // Cancels the job and suspends the invoking coroutine 
                                         // .. until the cancelled job is complete.
}

fun activeFlagCoroutine() = runBlocking {

    val job: Job = launch(Dispatchers.Default) {    // Dispatcher decide witch threads will use coroutine
        for (i in 1..2) {                           // func can't be here =(
            if (!isActive) {
                return@launch
            }
        }

        delay(1000)
    }

    job.cancelAndJoin()
}