pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code = &self.0;

        if code.trim().len() <= 1 {
            return false;
        }
    
        let num_val = code
            .replace(" ", "")
            .chars()
            .rev()
            .map(|ch| ch.to_digit(10))
            .enumerate()
            .try_fold(0u32, |acc, (i, n)| 
                match (i % 2, n) {
                    (0, Some(v)) => Some(acc + v),
                    (1, Some(v)) if v == 9 => Some(acc + v),
                    (1, Some(v)) => Some(acc + ((v * 2) % 9)),
                    _ => None
                });
    
        match num_val {
            Some(v) => v % 10 == 0,
            None =>  false
        }
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn where T: ToString {
    fn from(input: T) -> Self {
        Self(input.to_string())
    }
}
