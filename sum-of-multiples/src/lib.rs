pub fn sum_of_multiples(num: u32, vec: &[u32]) -> u32 {
    let mut sum :u32 = 0;

    for &v in vec {
        if v < num {
            sum += v;
        }
    }
    sum
}
