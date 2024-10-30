import kotlinx.coroutines.*

fun main() = runBlocking {

    val msgOne: Deferred<String> = async(start = CoroutineStart.LAZY) { 
        getMessageOne() 
    }

    val msgTwo: Deferred<String> = async(start = CoroutineStart.LAZY) { 
        getMessageTwo() 
    }

    println("${msgOne.await()} - ${msgTwo.await()}")
}

suspend fun getMessageOne(): String {
    delay(1000)
    return "Msg1"
}

suspend fun getMessageTwo(): String {
    delay(1000)
    return "Msg2"
}