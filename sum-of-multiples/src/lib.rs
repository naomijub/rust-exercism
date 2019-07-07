pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let filteres_factors = factors.iter()
            .filter(|c| c.to_owned() > &0u32)
            .map(|c| c.to_owned())
            .collect::<Vec<u32>>();
    if filteres_factors.len() == 0
        { return 0; }
    else if filteres_factors.first().unwrap() > &limit 
        { return 0; }
    else {
        let mut all_factors = filteres_factors.iter()
            .filter(|v| v.to_owned() < &limit)
            .map(|v| {
                let aux_vec: Vec<u32> = (0..limit).collect();
                aux_vec.iter()
                    .filter(|s| s.to_owned() % v == 0u32)
                    .map(|s| s.to_owned())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<u32>>();
        all_factors.sort();
        all_factors.dedup();
        all_factors.iter().fold(0,|acc, v| &acc + v.to_owned())
    }
}
