import kotlinx.coroutines.*

fun main() = runBlocking {
    val defCoroutine: Deferred<Int> = async {       // coroutine return Int
        println("def job")
        delay(1000)
        233444                                      // deffered job return value
    }

    val num: Int = defCoroutine.await()             // main coroutine is blocked here until `num` gets value (return from await())
}