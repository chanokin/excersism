static class SavingsAccount
{
    public static float InterestRate(decimal balance)
    {
        if (balance < 0) return 3.213f;

        if (balance < 1000) return 0.5f;

        if (balance < 5000) return 1.621f;

        return 2.475f;
    }

    public static decimal Interest(decimal balance)
    {
        return 0.01m * balance * (decimal)InterestRate(balance);
    }

    public static decimal AnnualBalanceUpdate(decimal balance)
    {
        return balance + Interest(balance);
    }

    public static int YearsBeforeDesiredBalance(decimal balance, decimal targetBalance)
    {
        if (targetBalance <= balance) return 0;
        
        return 1 + YearsBeforeDesiredBalance(AnnualBalanceUpdate(balance), targetBalance);
    }
}
