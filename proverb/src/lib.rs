pub fn build_proverb(list: &[&str]) -> String {
  match list {
      [] => String::new(),
      _ => make_phrases(list)
  }
}

fn make_phrases(list: &[&str]) -> String {
  list.windows(2)
      .map(|x| format!("For want of a {} the {} was lost.", x[0], x[1]))
      .chain(std::iter::once(format!(
        "And all for the want of a {}.",
        list[0]
    )))
      .collect::<Vec<_>>()
      .join("\n")
}