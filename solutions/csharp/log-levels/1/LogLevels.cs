static class LogLine
{
    public static string Message(string logLine) =>
        logLine.Split("]: ", StringSplitOptions.TrimEntries)[1];

    public static string LogLevel(string logLine) =>
        logLine.Split("]: ", StringSplitOptions.TrimEntries)[0].Split("[")[1].ToLower();
    
    public static string Reformat(string logLine)
    {   
        var error_message_split = logLine.Split("]: ", StringSplitOptions.TrimEntries);
        var message = error_message_split[1];
        var error = error_message_split[0].Split("[")[1].ToLower();
        return $"{message} ({error})";
    }
}
