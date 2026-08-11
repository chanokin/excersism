public interface IRemoteControlCar {
    int DistanceTravelled { get; set; }
    void Drive();
} 

public class ProductionRemoteControlCar : IRemoteControlCar, IComparable<ProductionRemoteControlCar>
{
    public int DistanceTravelled { get; set; }
    public int NumberOfVictories { get; set; }

    public void Drive() {
        DistanceTravelled += 10;
    }

    public int CompareTo(ProductionRemoteControlCar other)
    {
        // If other is not a valid object reference, this instance is greater.
        if (other == null) return 1;

        // The temperature comparison depends on the comparison of
        // the underlying Double values.
        return this.NumberOfVictories.CompareTo(other.NumberOfVictories);
    }

}

public class ExperimentalRemoteControlCar : IRemoteControlCar
{
    public int DistanceTravelled { get; set; }

    public void Drive()
    {
        DistanceTravelled += 20;
    }
}

public static class TestTrack
{
    public static void Race(IRemoteControlCar car)
    {
        car.Drive();
    }

    public static List<ProductionRemoteControlCar> GetRankedCars(ProductionRemoteControlCar prc1,
        ProductionRemoteControlCar prc2)
    {
        List<ProductionRemoteControlCar> list = new List<ProductionRemoteControlCar>();
        if (prc1.NumberOfVictories > prc2.NumberOfVictories) {
            list.Add(prc2);
            list.Add(prc1);
        } else {
            list.Add(prc1);
            list.Add(prc1);            
        }

        return list;
    }
}
