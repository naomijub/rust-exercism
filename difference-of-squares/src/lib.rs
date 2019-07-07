pub fn square_of_sum(n: u32) -> u32 {
    let vec = (1u32..=n).collect::<Vec<u32>>();
    vec.into_iter().fold(0u32,|acc,n| acc + n).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let vec = (1u32..=n).collect::<Vec<u32>>();
    vec.into_iter().map(|n| n.pow(2)).fold(0u32,|acc,n| acc + n)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
