use orders::{get_highest_card};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum HandOrder {
    HighCard = 1,
    // Pair = 2,
}

fn determine_hand_type<'a>(hand: &'a str) -> HandOrder {
  HandOrder::HighCard
}

fn determine_best_game(hands: &mut Vec<HandOrder>) -> &HandOrder {
  hands.sort();
  hands.reverse();
  hands.first().unwrap()
}

fn filter_hands_by_winner<'a>(hands: Vec<(HandOrder, &'a str)>, order: HandOrder) -> Vec<&'a str> {
  hands.iter().filter(|h| h.0 == order).map(|h| h.1).collect::<Vec<&'a str>>()
}



fn get_best<'a>(hands: Vec<&'a str>, order: &HandOrder ) -> Vec<&'a str> {
  match order.to_owned() {
      HandOrder::HighCard => get_highest_card(hands),
      // HandOrder::Pair => get_highest_pair(hands),
  }
}

fn return_best_hands<'a>(hands: Vec<(HandOrder, &'a str)>) -> Vec<&'a str> {
  let mut orders = hands.clone().into_iter()
                  .map(|h| h.0)
                  .collect::<Vec<HandOrder>>();
  let best_game = determine_best_game(&mut orders);
  let high_hands = match best_game {
    HandOrder::HighCard => filter_hands_by_winner(hands, HandOrder::HighCard),
    // HandOrder::Pair => filter_hands_by_winner(hands, HandOrder::Pair),
  };
  get_best(high_hands, best_game)                
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
  if hands.len() == 1 { return Some(hands.to_owned()); }
  let hands_type = hands.into_iter()
    .map(|h| (determine_hand_type(h), h.to_owned()))
    .collect::<Vec<(HandOrder, &'a str)>>();

  Some(return_best_hands(hands_type))
}

mod orders {
  pub fn get_highest_card<'a> (hands: Vec<&'a str>)  -> Vec<&'a str> {
    let mut vec: Vec<(i32,&'a str)> = Vec::new();
    vec.push((-1, ""));

    hands.into_iter()
      .map(|h| match h {
        hand if hand.contains("A") => (14, hand),
        hand if hand.contains("K") => (13, hand),
        hand if hand.contains("Q") => (12, hand),
        hand if hand.contains("J") => (11, hand),
        hand if hand.contains("10") => (10, hand),
        hand if hand.contains("9") => (9, hand),
        hand if hand.contains("8") => (8, hand),
        hand if hand.contains("7") => (7, hand),
        hand if hand.contains("6") => (6, hand),
        hand if hand.contains("5") => (5, hand),
        hand if hand.contains("4") => (4, hand),
        hand if hand.contains("3") => (3, hand),
        hand if hand.contains("2") => (2, hand),
        _ => (0, "")
      })
      .fold(vec, |mut acc, h| {
        if h.0 > acc[0].0 {
          acc.retain(|&x| x.0 >= h.0);
          acc.push((h.0, h.1));
        }
        acc
      })
      .iter()
      .map(|h| h.1)
      .collect::<Vec<&'a str>>()
  }
}