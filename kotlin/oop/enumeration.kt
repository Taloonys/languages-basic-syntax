enum class WeekDays {
    MONDAY,
    TUESDAY,
    // ..
    SUNDAY
}


enum class WeekDays1(val v: Int) {
    MONDAY(1),
    TUESDAY(2),
    // ..
    SUNDAY(7)
}


interface IStatus {
    fun cashLimit(): Int
}

enum class AccountStatus(val high_stat: Boolean = false, val money: Int) 
    : IStatus 
{
    VIP(true, 5000)     { override fun cashLimit(): Int = 15000 },
    SIMPLE(false, 500)  { override fun cashLimit(): Int = 8000 }
}


fun main() {
    println("""
            ${AccountStatus.VIP.high_stat}
            ${AccountStatus.SIMPLE.cashLimit()}
            """)
}