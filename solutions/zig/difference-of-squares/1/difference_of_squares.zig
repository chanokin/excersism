pub fn squareOfSum(number: usize) usize {
    // this is can be done with Gauss
    var sum: usize = 0;
    for (1 .. number + 1) |n| {
        sum += n;
    }

    // square sum
    sum *= sum;
    return sum;
}

pub fn sumOfSquares(number: usize) usize {
    var sum: usize = 0;
    for (1 .. number + 1) |n| {
        sum += (n * n);
    }
    return sum;
}

pub fn differenceOfSquares(number: usize) usize {
    const squared_sum: usize = squareOfSum(number);
    const sum_of_squares: usize = sumOfSquares(number);
    return squared_sum - sum_of_squares;
}
