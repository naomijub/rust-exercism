pub fn build_proverb(list: &[&str]) -> String {
  match list.clone().len() {
    x if x <= 0 => String::new(),
    x if x > 0 => proverb_generator(list),
    _ => String::new()
  }
}

fn proverb_generator(list: &[&str]) -> String {
  let mut next_item_counter = 1;
  let items_to_take = list.clone().len() - 1;

  let all_items = list.clone().into_iter().map(move |item| {
    let next = list[next_item_counter];
    if next_item_counter + 1 < list.len() { next_item_counter += 1}
    format!("For want of a {} the {} was lost.\n", item, next)
  }).take(items_to_take).collect::<String>();

  all_items + &format!("And all for the want of a {}.", list.first().unwrap())
}