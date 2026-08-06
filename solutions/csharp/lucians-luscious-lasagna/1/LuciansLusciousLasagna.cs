class Lasagna
{
    public int ExpectedMinutesInOven() {
        return 40;
    }

    public int RemainingMinutesInOven(int elapsed_time_minutes) {
        int remaining = ExpectedMinutesInOven() - elapsed_time_minutes;
        return Math.Max(0, remaining);
    }

    public int PreparationTimeInMinutes(int number_of_layers) {
        return number_of_layers * 2;        
    }

    public int ElapsedTimeInMinutes(int number_of_layers, int time_in_oven_minutes) {
        return PreparationTimeInMinutes(number_of_layers) + time_in_oven_minutes;
    } 
}
