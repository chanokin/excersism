object Acronym {
    fun generate(phrase: String) : String {
        val words = phrase.replace("_", "").replace("-", " ").split(Regex("\\s+"))
        val acronym = words.map{it[0].uppercaseChar()}
        
        return acronym.joinToString("")
    }
}
