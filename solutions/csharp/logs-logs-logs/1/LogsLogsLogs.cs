enum LogLevel {
    Unknown,
    Trace,
    Debug,
    Info = 4,
    Warning,
    Error,
    Fatal = 42
}

static class LogLine
{
    public static LogLevel ParseLogLevel(string logLine)
    {
        string level = logLine.Split("]: ")[0].Split("[")[1].ToLower();

        switch (level) {
            case "trc": return LogLevel.Trace;
            case "dbg": return LogLevel.Debug;
            case "inf": return LogLevel.Info;
            case "wrn": return LogLevel.Warning;
            case "err": return LogLevel.Error;
            case "ftl": return LogLevel.Fatal;
            default: return LogLevel.Unknown;     
        }
    }

    public static string OutputForShortLog(LogLevel logLevel, string message)
    {   
        int level = (int)logLevel;
        
        return $"{level}:{message}";
    }
}
