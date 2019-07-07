pub fn factors(n: u64) -> Vec<u64> {
    match n {
      1 => vec![],
      2 => vec![2],
      x if x > 2 => factorize(x), 
      _ => vec![]
    }
}

fn factorize(n: u64) -> Vec<u64> {
  let mut vec: Vec<u64> = Vec::new();
  refactorize(n, 2, &mut vec)
}

fn refactorize(n: u64, current: u64, vec: &mut Vec<u64>) -> Vec<u64> {
  if n <= 1 {
    return vec.to_owned();
  } else {
    if n % current == 0 {
      vec.push(current);
      return refactorize(n / current, current, vec);
    } else {
      let next = if current % 2 == 0 {current + 1} else {current + 2}; 
      return refactorize(n, next, vec);
    }
  }
}
