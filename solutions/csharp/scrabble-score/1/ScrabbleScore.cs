public static class ScrabbleScore
{
    public static int Score(string input)
    {    
        int sum = 0;
        foreach (char ch in input.ToLower()) {
            if ( "aeioulnrst".Contains(ch) ) {
                sum += 1;
                continue;
            }

            if ("dg".Contains(ch)) {
                sum += 2;
                continue;
            }

            if ("bcmp".Contains(ch)) {
                sum += 3;
                continue;
            }

            if ("fhvwy".Contains(ch)) {
                sum += 4;
                continue;
            }

            if (ch == 'k') {
                sum += 5;
                continue;
            }

            if ("jx".Contains(ch)) {
                sum += 8;
                continue;
            }

            if ("qz".Contains(ch)) {
                sum += 10;
            }            
        }

        return sum;
    }
}