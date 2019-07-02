use orders::{get_highest_card, get_highest_pair, determine_hand_group};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum HandOrder {
    HighCard = 1,
    Pair = 2,
}

fn determine_hand_type<'a>(hand: &'a str) -> HandOrder {
  let values_map = determine_hand_group(hand);
  
  match values_map.values() {
    ref m if m.len() == 5 =>   HandOrder::HighCard,
    ref m if m.len() == 4 =>   HandOrder::Pair,
    _ => HandOrder::HighCard
  }
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
      HandOrder::Pair => get_highest_pair(hands),
  }
}

fn return_best_hands<'a>(hands: Vec<(HandOrder, &'a str)>) -> Vec<&'a str> {
  let mut orders = hands.clone().into_iter()
                  .map(|h| h.0)
                  .collect::<Vec<HandOrder>>();
  let best_game = determine_best_game(&mut orders);
  let high_hands = match best_game {
    HandOrder::HighCard => filter_hands_by_winner(hands, HandOrder::HighCard),
    HandOrder::Pair => filter_hands_by_winner(hands, HandOrder::Pair),
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
  use std::collections::HashMap;

  pub fn determine_hand_group<'a>(hand: &'a str ) -> HashMap<&'a str, i32> {
    let mut values_map: HashMap<&'a str, i32> = HashMap::new();
      hand.split(" ")
          .into_iter()
          .for_each(|c| 
            match c {
              c if c.contains("A") => *values_map.entry("A").or_insert(0) += 1,
              c if c.contains("K") => *values_map.entry("K").or_insert(0) += 1,
              c if c.contains("Q") => *values_map.entry("Q").or_insert(0) += 1,
              c if c.contains("J") => *values_map.entry("J").or_insert(0) += 1,
              c if c.contains("10") => *values_map.entry("10").or_insert(0) += 1,
              c if c.contains("9") => *values_map.entry("9").or_insert(0) += 1,
              c if c.contains("8") => *values_map.entry("8").or_insert(0) += 1,
              c if c.contains("7") => *values_map.entry("7").or_insert(0) += 1,
              c if c.contains("6") => *values_map.entry("6").or_insert(0) += 1,
              c if c.contains("5") => *values_map.entry("5").or_insert(0) += 1,
              c if c.contains("4") => *values_map.entry("4").or_insert(0) += 1,
              c if c.contains("3") => *values_map.entry("3").or_insert(0) += 1,
              c if c.contains("2") => *values_map.entry("2").or_insert(0) += 1,
              _ => *values_map.entry("0").or_insert(0) += 0
            });
    values_map
  }

  pub fn get_highest_card<'a> (hands: Vec<&'a str>)  -> Vec<&'a str> {
    let mut vec: Vec<(i32,&'a str)> = Vec::new();
    vec.push((-1, ""));

    let highest_hands = hands.into_iter()
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
        if h.0 >= acc[0].0 {
          acc.retain(|&x| x.0 >= h.0);
          acc.push((h.0, h.1));
        }
        acc
      });
     ranked_hands(highest_hands)
  }

  pub fn get_highest_pair<'a> (hands: Vec<&'a str>)  -> Vec<&'a str> {
    let values_map = values_map(&hands);

    let max_pair = max_pair(&values_map);

    hands.iter()
          .filter(|h| h.split("")
              .into_iter()
              .filter(|c| c.contains(max_pair))
              .fold(0,|acc, _c| acc + 1) >= 2)
          .map(|h| h.to_owned())
          .collect::<Vec<&'a str>>()
  }

  fn ranked_hands<'a>(vec: Vec<(i32, &'a str)>) -> Vec<&'a str> {
    let mut ranked_vec: Vec<(i32,&'a str)> = Vec::new();
    ranked_vec.push((-1, ""));

    vec.iter()
      .map(|h| h.1)
      .collect::<Vec<&'a str>>()
      .iter()
      .map(|h| (h.split(" ")
                .into_iter()
                .map(|c| match_card_value(c))
                .fold(0,|acc,c| acc + c), h))
      .fold(ranked_vec, |mut acc, h| {
        if h.0 >= acc[0].0 {
          acc.retain(|&x| x.0 >= h.0);
          acc.push((h.0, h.1));
        }
        acc
      })
      .iter()
      .map(|h| h.1)
      .collect::<Vec<&'a str>>()
  }

  fn match_card_value<'a>(c: &'a str) -> i32 {
    match c {
      c if c.contains("A") => 60547001,
      c if c.contains("K") => 12109401,
      c if c.contains("Q") => 2421881,
      c if c.contains("J") => 484376,
      c if c.contains("10") => 96876,
      c if c.contains("9") => 19376,
      c if c.contains("8") => 3876,
      c if c.contains("7") => 776,
      c if c.contains("6") => 155,
      c if c.contains("5") => 31,
      c if c.contains("4") => 6,
      c if c.contains("3") => 1,
      c if c.contains("2") => 0,
      _ => 0
    }
  }

  fn values_map<'a> (hands: &Vec<&'a str>) -> Vec<(&'a str, i32)> {
    hands.iter()
        .map(|h| determine_hand_group(h))
        .map(|h| h.keys().into_iter()
            .flat_map(|&k| h.get(&k).map(|&v| (k,v)).into_iter())
            .fold(Vec::new(),|mut acc,v| {acc.push(v); acc}))
        .map(|h| h.iter()
            .fold(("",0),|acc, c| {if c.1 > acc.1 { return (c.0,c.1); } acc}))
        .collect::<Vec<(&'a str, i32)>>()
  }

  fn max_pair<'a>(hand_map: &Vec<(&'a str, i32)>) -> &'a str {
    hand_map.iter()
        .map(|c| match c {
            card if card.0.contains("A") => (14, card.0),
            card if card.0.contains("K") => (13, card.0),
            card if card.0.contains("Q") => (12, card.0),
            card if card.0.contains("J") => (11, card.0),
            card if card.0.contains("10") => (10, card.0),
            card if card.0.contains("9") => (9, card.0),
            card if card.0.contains("8") => (8, card.0),
            card if card.0.contains("7") => (7, card.0),
            card if card.0.contains("6") => (6, card.0),
            card if card.0.contains("5") => (5, card.0),
            card if card.0.contains("4") => (4, card.0),
            card if card.0.contains("3") => (3, card.0),
            card if card.0.contains("2") => (2, card.0),
            _ => (0, "0")
        })
        .fold((0,""), |acc, c| {if c.0 > acc.0 { return (c.0, c.1);} acc}).1
  }
}