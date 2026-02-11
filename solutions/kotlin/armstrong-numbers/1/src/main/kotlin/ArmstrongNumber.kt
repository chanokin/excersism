import kotlin.math.*

object ArmstrongNumber {

    fun check(input: Int): Boolean {

        var current_input: Int = input;
        val n_digits: Int = 1 + floor(log10( input.toDouble() )).toInt();

        if (n_digits == 1) {
            return true;
        }
        
        val ten: Double = 10.0;
        
        var sum_armstrong: Int = 0;
        for (_power in n_digits downTo 1) {
            val base: Double = floor(current_input / ten.pow(_power - 1));
            val armstrong_digit: Int = base.pow(n_digits).toInt();
            current_input -= base.times(ten.pow(_power - 1)).toInt();
            sum_armstrong += armstrong_digit;
        }
        
        return input == sum_armstrong; 
    }

}
