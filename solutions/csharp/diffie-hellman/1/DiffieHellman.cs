using System.Numerics;

public static class DiffieHellman
{
    public static BigInteger PrivateKey(BigInteger primeP) 
    {    
        var min_val = new BigInteger(2);
        var rng = new System.Random();
        return new BigInteger(rng.NextDouble() * (double) (primeP - min_val) + 2.0d);
    }

    public static BigInteger PublicKey(BigInteger primeP, BigInteger primeG, BigInteger privateKey) =>
        BigInteger.ModPow(primeG, privateKey, primeP);

    public static BigInteger Secret(BigInteger primeP, BigInteger publicKey, BigInteger privateKey) =>
        BigInteger.ModPow(publicKey, privateKey, primeP);
}