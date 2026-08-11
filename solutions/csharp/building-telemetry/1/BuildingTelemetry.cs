public class RemoteControlCar
{
    private int batteryPercentage = 100;
    private int distanceDrivenInMeters = 0;
    private string[] sponsors = new string[0];
    private int latestSerialNum = 0;

    public void Drive()
    {
        if (batteryPercentage > 0)
        {
            batteryPercentage -= 10;
            distanceDrivenInMeters += 2;
        }
    }

    public void SetSponsors(params string[] sponsors)
    {
        this.sponsors = new string[sponsors.Length];
        int index = 0;
        foreach (string sponsor in sponsors) {
            this.sponsors[index] = sponsor;
            index += 1;
        }
    }

    public string DisplaySponsor(int sponsorNum) => this.sponsors[sponsorNum];

    public bool GetTelemetryData(ref int serialNum,
        out int batteryPercentage, out int distanceDrivenInMeters)
    {
        batteryPercentage = this.batteryPercentage;
        distanceDrivenInMeters = this.distanceDrivenInMeters;

        bool orderly = serialNum > this.latestSerialNum;
        
        if (orderly) {
            this.latestSerialNum = serialNum;
        }
        else {
            serialNum = this.latestSerialNum;
            batteryPercentage = -1;
            distanceDrivenInMeters = -1;
        }
        return orderly;
    }

    public static RemoteControlCar Buy()
    {
        return new RemoteControlCar();
    }
}

public class TelemetryClient
{
    private RemoteControlCar car;

    public TelemetryClient(RemoteControlCar car)
    {
        this.car = car;
    }

    public string GetBatteryUsagePerMeter(int serialNum)
    {
        int batteryPercentage = 0;
        int distanceDrivenInMeters = 0;
        var result = this.car.GetTelemetryData(ref serialNum, 
                                               out batteryPercentage, 
                                               out distanceDrivenInMeters);

        if (!result || distanceDrivenInMeters == 0) {
            return "no data";
        }

        var usage_per_meter = (100 - batteryPercentage) / distanceDrivenInMeters;

        return $"usage-per-meter={usage_per_meter}";
    }
}
