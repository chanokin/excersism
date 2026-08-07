static class AssemblyLine
{
    public static double SuccessRate(int speed)
    {
        if (speed == 0) return 0.0;

        if (speed > 0 && speed < 5) return 1.0;

        if (speed > 4 && speed < 9) return 0.9;

        if (speed > 8 && speed < 10) return 0.8;

        if (speed == 10) return 0.77;

        return 0.0;
    }
    
    public static double ProductionRatePerHour(int speed) => 221.0 * speed * SuccessRate(speed);

    public static int WorkingItemsPerMinute(int speed) => (int)ProductionRatePerHour(speed) / 60 ;

}
