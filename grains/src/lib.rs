pub fn square(s: u32) -> u64 {
    if s < 1u32 || s > 64u32 {panic!("Square must be between 1 and 64");}
    2u64.pow(s - 1u32)
}

pub fn total() -> u64 {
    (1u32..=64u32).collect::<Vec<u32>>()
      .into_iter()
      .map(|s| square(s))
      .fold(0u64,|acc,s| acc + s)
}
