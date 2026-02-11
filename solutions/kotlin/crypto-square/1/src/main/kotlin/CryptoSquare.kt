import kotlin.math.*

object CryptoSquare {

    fun ciphertext(plaintext: String): String {
        if (plaintext.length == 0)
            return plaintext
        
        var alphanumerical: String = plaintext.lowercase().filter{ it.isLetterOrDigit() }
        val n_chars = alphanumerical.length

        
        val n_cols = ceil(sqrt(n_chars.toDouble())).toInt()
        val n_rows = (n_chars + n_cols - 1) / n_cols

        var reordered: String = ""
        for (col in 0 until n_cols) {
            for (row in 0 until n_rows) {
                val index = row * n_cols + col
                var char_at: Char = ' '
                if (index < n_chars){
                    char_at = alphanumerical[index]
                }
                reordered = reordered + char_at
                
            }
            if (col < (n_cols - 1)){
                reordered = reordered + ' '
            }
        }
        
        return reordered
    }

}
