fun main() {
    var lst = ArrayList<Int>().let {        // also `mutableListOf`, `arrayListOf`
        it.add(1)
        it.add(2)

        it.remove(2)
    }

    var hMap = HashMap<Int, String>().let {   // also `hashMapOf`, `mutableMap`
        it.put(1, "One")
    }

    var simpleSet   = mutableSetOf<Int>(2, 3, 3, 4)

    var hashSet     = hashSetOf<Int>(2, 3, 3, 4)
}