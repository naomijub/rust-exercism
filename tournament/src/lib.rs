use std::collections::HashMap;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Default, Eq, PartialEq)]
struct Team {
    name: String,
    matches: u8,
    wins: u8,
    draws: u8,
    losses: u8,
    points: u16,
}

impl Team {
    fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    fn win(&mut self) {
        self.wins += 1;
        self.matches += 1;
        self.points += 3;
    }

    fn lose(&mut self) {
        self.losses += 1;
        self.matches += 1;
    }

    fn draw(&mut self) {
        self.draws += 1;
        self.matches += 1;
        self.points += 1;
    }

    fn add_match(&mut self, result: &MatchResult) {
        match result {
            MatchResult::Win => self.win(),
            MatchResult::Loss => self.lose(),
            MatchResult::Draw => self.draw(),
        }
    }
}

impl From<&Team> for String {
    fn from(origin: &Team) -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            origin.name, origin.matches, origin.wins, origin.draws, origin.losses, origin.points
        )
    }
}

enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl From<&str> for MatchResult {
    fn from(origin: &str) -> MatchResult {
        match origin {
            "win" => MatchResult::Win,
            "loss" => MatchResult::Loss,
            "draw" => MatchResult::Draw,
            _ => panic!(),
        }
    }
}

impl MatchResult {
    fn reverse(&self) -> Self {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Loss => MatchResult::Win,
            MatchResult::Draw => MatchResult::Draw,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut scores: HashMap<String, Team> = HashMap::new();
    match_results.lines().for_each(|line| {
        let frags: Vec<&str> = line.split(";").collect();
        let home = frags[0];
        let away = frags[1];
        let result = frags[2].into();
        scores
            .entry(home.into())
            .or_insert(Team::new(home.into()))
            .add_match(&result);
        scores
            .entry(away.into())
            .or_insert(Team::new(away.into()))
            .add_match(&result.reverse());
    });

    let mut score_values: Vec<&Team> = scores.values().collect();
    score_values.sort_by(|a, b| b.points.cmp(&a.points).then_with(|| a.name.cmp(&b.name)));

    vec![String::from(HEADER)]
        .into_iter()
        .chain(score_values.into_iter().map(|t| t.into()))
        .collect::<Vec<String>>()
        .join("\n")
}