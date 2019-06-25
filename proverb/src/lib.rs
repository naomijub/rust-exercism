pub fn build_proverb(list: &[&str]) -> String {
  let mut counter = 1;
  let items_to_take = list.clone().len() - 1;

  let all_items = list.clone().into_iter().map(move |item| {
    let next = list[counter];
    if counter + 1 < list.len() { counter += 1}
    format!("For want of a {} the {} was lost.\n", item, next)
  }).take(items_to_take).collect::<String>();

  all_items + &format!("And all for the want of a {}.", list.first().unwrap())
}
