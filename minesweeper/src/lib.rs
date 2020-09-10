pub fn annotate(m: &[&str]) -> Vec<String> {
    let mut matrix = m
        .iter()
        .map(|c| {
            c.chars()
                .map(|ch| if ch == ' ' { 0 } else { i8::MIN })
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    matrix
        .iter()
        .enumerate()
        .flat_map(|(l_pos, line)| {
            line.iter()
                .enumerate()
                .filter(|e| *e.1 < 0i8)
                .map(move |(c_pos, _)| (l_pos, c_pos))
        })
        .collect::<Vec<(usize, usize)>>()
        .into_iter()
        .flat_map(|(l_pos, c_pos)| {
            (l_pos.saturating_sub(1)..=l_pos + 1)
                .flat_map(move |l| (c_pos.saturating_sub(1)..=c_pos + 1).map(move |c| (l, c)))
        })
        .map(|(l, c)| matrix.get_mut(l)?.get_mut(c).map(|el| *el += 1))
        .fold((), |acc, _| acc);

    matrix
        .iter()
        .map(|e| {
            e.iter()
                .map(|c| match c {
                    i8::MIN..=-1 => '*',
                    0 => ' ',
                    num => (48 + (*num) as u8) as char,
                })
                .collect::<String>()
        })
        .collect()
}
