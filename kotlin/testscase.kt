import org.junit.Assert
import org.junit.Test


fun main() {

    suspend fun summ(x: Int, y: Int): Int = x + y

    @Test                                               // allows to run isolated Tests (it's more like IDE integration)
    fun testCase() {
        Assert.assertEquals(3, summ(1, 2)) 
    }


}