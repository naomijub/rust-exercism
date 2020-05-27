use rayon::prelude::*;

fn is_prime(n: u32) -> bool {
    !(2u32..n.div_euclid(2) + 1)
        .collect::<Vec<u32>>()
        .par_iter()
        .any(|i| n % i == 0)
}

pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2u32,
        n => (1u32..)
            .filter(|num| is_prime(*num))
            .nth((n + 1) as usize)
            .unwrap(),
    }
}
