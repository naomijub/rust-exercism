use std::collections::BTreeSet;

pub fn sum_of_multiples(num: u32, vec: &[u32]) -> u32 {
    let mut multiples: BTreeSet<u32> = BTreeSet::new();

    for &v in vec {
        let mut factor = 2;
        let mut x = v;
        while x < num {
            multiples.insert(x);
            x = v * factor;
            factor += 1;
        }
    }

    multiples.iter().sum()
}
