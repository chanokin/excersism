// Please implement the `ComputationError.IllegalArgument` error.

pub const ComputationError = error{
    IllegalArgument,
};

pub fn steps(number: usize) anyerror!usize {
    if (number == 0) {
        return ComputationError.IllegalArgument;
    }
    var x: usize = number;
    var step: usize = 0;

    while (true) {
        if (x == 1) {
            return step;
        }

        if ((x % 2) == 0) { // even
            x /= 2;
        } else { // odd
            x = 3 * x + 1;
        }
        step += 1;
    }
}
