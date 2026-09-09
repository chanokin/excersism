public static class Bob
{
    public static string Response(string statement)
    {    
        // char[] charsToTrim = {};
        statement = statement.Trim();
        int len = statement.Length;

        if (len == 0) {
            return "Fine. Be that way!";
        }

        bool non_alpha = true;

        foreach (char ch in statement) {
            non_alpha = non_alpha && ! Char.IsLetter(ch);
        }
        
        if (statement.EndsWith('?')) {
            string sub = statement.Substring(0, len - 1);
            if (non_alpha) {
                return "Sure.";
            }
            if (sub.ToUpper() == sub) {
                return "Calm down, I know what I'm doing!";
            }
            
            return "Sure.";
        }

        if (!non_alpha && statement.ToUpper() == statement) {
            return "Whoa, chill out!";
        }

        return "Whatever.";
    }
}