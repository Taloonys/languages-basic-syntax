import kotlin.concurrent.thread

fun main() {                            // ofc there is an main execution thread
    thread {
        Thread.sleep(1000)
        println("Thread empty Work")
    }    
}