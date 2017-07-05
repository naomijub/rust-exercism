pub fn square_of_sum(n: i32) -> i32 {
    let sum = n * (n + 1) / 2;
    sum * sum
}

pub fn sum_of_squares(n: i32) -> i32 {
    (1..n + 1).map(|x| x * x).fold(0, |accumulator, x| accumulator + x)
}

pub fn difference(x: i32) -> i32 {
    0
}
