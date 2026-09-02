public static class RotationalCipher
{
    public static string Rotate(string text, int shiftKey)
    {
        if (shiftKey == 0 || shiftKey == 26) {
            return text;
        }

        char[] rotated = new char[text.Length];
        int index = 0;
        foreach (char c in text) {
            if (System.Char.IsLetter(c) == false) {
                rotated[index] = c;
                index += 1;
                continue;
            }

            int rotation_base = System.Char.IsLower(c) ? 97 : 65;
            int rotated_char = ((int)(c) - rotation_base + shiftKey) % 26;
            rotated_char += rotation_base;
            
            rotated[index] = (char)(rotated_char);
            index += 1;
        }
        
        return new string(rotated);
    }
}