public class Clock
{
    int _hours;
    int _minutes;
    private int mod(int x, int y) {
        int rem = x % y;
        return rem < 0 ? rem + y : rem;
    }
    private int idiv(int x, int y) {
        int d = x / y;

        if (x < 0) {
            d -= 1;
        }

        if ( x == -60 ) {
            d += 1;
        }

        return d;
    }
    
    public Clock(int hours, int minutes)
    {
        _minutes = mod(minutes, 60);
        _hours = mod((hours + idiv(minutes, 60)), 24) ;
    }

    public Clock Add(int minutesToAdd)
    {
        int minutes = _minutes + minutesToAdd;
        int hours = _hours + idiv(minutes, 60);

        return new Clock(mod(hours, 24), 
                         mod(minutes, 60));
    }

    public Clock Subtract(int minutesToSubtract)
    {
        int minutes = _minutes - minutesToSubtract;
        int hours = _hours + idiv(minutes, 60);

        return new Clock(mod(hours, 24), 
                         mod(minutes, 60));

    }

    public string ToString() {
        return $"{_hours:D2}:{_minutes:D2}";
    }
    
    public override bool Equals(object? obj) => obj is Clock other && Equals(other);
    
    public bool Equals(Clock? other) => other is not null && _hours == other._hours && _minutes == other._minutes;
    
}
