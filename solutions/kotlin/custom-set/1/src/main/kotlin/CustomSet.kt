class CustomSet() {
    var data : MutableList<Int> = mutableListOf<Int>()
    // TODO: implement proper constructor

    constructor(vararg values: Int) : this() {
        values.forEach{
            this.data.add(it)
        }
    }
    
    fun isEmpty(): Boolean {
        return data.size == 0
    }

    fun isSubset(other: CustomSet): Boolean {
        if (!this.isEmpty() and other.isEmpty()) {
            return false
        }
        
        if (this.isEmpty() and !other.isEmpty()) {
            return true
        }

        for (element in this.data) {
            if( !other.contains(element) ) {
                return false
            }
        }

        return true
    }

    fun isDisjoint(other: CustomSet): Boolean {
        for (elem in this.data) {
            if (other.contains(elem)){
                return false
            }
        }
        return true
    }

    fun contains(other: Int): Boolean {
        return this.data.contains(other)
    }

    fun intersection(other: CustomSet): CustomSet {
        var output = CustomSet()
        for (elem in this.data) {
            if (other.contains(elem)){
                output.add(elem)
            }
        }

        return output

    }

    fun add(other: Int) {
        if (!this.contains(other)) {
            this.data.add(other)
        }
    }

    operator override fun equals(other: Any?): Boolean {
        val otherSet = other as CustomSet
        if (this.isEmpty() and otherSet.isEmpty()) {
            return true
        }

        if (this.isEmpty() or otherSet.isEmpty()) {
            return false
        }

        if (this.data.size != other.data.size) {
            return false
        }
        
        return this.isSubset(otherSet)
    }

    operator fun plus(other: CustomSet): CustomSet {
        var output = CustomSet()
        for (elem in this.data) {
            output.add(elem)
        }
        for (elem in other.data) {
            output.add(elem)
        }
        return output
    }

    operator fun minus(other: CustomSet): CustomSet {
        var output = CustomSet()
        for (elem in this.data) {
            if (!other.contains(elem)){
                output.add(elem)
            }
        }

        return output
    }
}
