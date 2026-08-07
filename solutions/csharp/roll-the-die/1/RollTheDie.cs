public class Player
{
    public int RollDie()
    {
        var rng = new System.Random();
        // non-inclusive range end
        return rng.Next(1, 19);
    }

    public double GenerateSpellStrength()
    {
        var rng = new System.Random();
        // non-inclusive range end
        return rng.NextDouble() * 100.0d;
    }
}
