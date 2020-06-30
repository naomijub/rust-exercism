pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|number| factors.iter().filter(|x| **x > 0).any(|x| number % x == 0))
        .sum()
}