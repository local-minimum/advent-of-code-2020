use std::collections::{HashSet, VecDeque};
use std::fs;

fn load_demo() -> String {
    r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#.to_string()
}

fn load_demo2() -> String {
    r#"Player 1:
43
19

Player 2:
2
29
14"#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Deck {
    player: String,
    cards: VecDeque<usize>,
}

impl Deck {
    fn from_string(data: String) -> Vec<Deck> {
        let mut decks = vec![];
        let mut player: String = "".to_string();
        let mut cards: VecDeque<usize> = VecDeque::new();
        for line in data.lines() {
            if line.starts_with("Player") {
                player = line.trim().replace(":", "");                
            } else if line.is_empty() {
                decks.push(Deck{player, cards});
                player = "".to_string();
                cards = VecDeque::new();
            } else {
                let card: usize = line.parse().unwrap();
                cards.push_back(card);
            }
        }
        if !player.is_empty() {
            decks.push(Deck{player, cards});
        }
        decks
    }

    fn get_card(&mut self) -> Option<usize> {
        self.cards.pop_front()
    }

    fn gain_cards(&mut self, own_card: usize, opponent_card: usize) {
        self.cards.push_back(own_card);
        self.cards.push_back(opponent_card);
    }

    fn return_card(&mut self, card: usize) {
        self.cards.push_front(card);        
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for (idx, card) in self.cards.iter().enumerate() {
            score += card * (self.cards.len() - idx);
        }
        score
    }

    fn cards(&self) -> usize {
        self.cards.len()
    }

    fn cards_to_string(&self, held_card: &usize) -> String {
        if *held_card == 0 {
            self.cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ")
        }
        else if self.cards() > 0 {
            format!(
                "{}, {}",
                held_card,
                self.cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "),
            )
        } else {

            held_card.to_string()
        }
    }

    fn trim_deck_clone(&self, size: &usize) -> Deck {
        let mut cards = self.cards.clone();
        while cards.len() > *size {
            cards.pop_back();
        }
        Deck{
            player: self.player.clone(),
            cards 
        }
    }
}

fn play(mut decks: Vec<Deck>) {
    loop {
        match decks[0].get_card() {
            Some(player_one) => {
                match decks[1].get_card() {
                    Some(player_two) => {
                        if player_one > player_two {
                            decks[0].gain_cards(player_one, player_two);
                        } else {
                            decks[1].gain_cards(player_two, player_one);
                        }
                    },
                    None => {
                        decks[0].return_card(player_one);
                        break;
                    }
                }
            },
            None => {
                break;
            }        
        }        
    }

    for deck in decks.iter() {
        println!("{} scores: {}", deck.player, deck.score());
    }
}

fn play_recursive(mut d1: Deck, mut d2: Deck, game: usize, verbose: &bool) -> usize {
    let mut round = 1;
    let mut player_history: Vec<HashSet<VecDeque<usize>>> = vec![
        HashSet::new(),
        HashSet::new(),
    ];
    if *verbose {
        println!("\n=== Game {} ===", game);
    }
    loop {
        if player_history[0].contains(&d1.cards) {
            if *verbose {
                println!("Player 1 wins round {} in game {} on infinite recursion", round, game);
                println!("{}'s deck {}", d1.player, d1.cards_to_string(&0));
            }
            return 0;
        } else {
            player_history.get_mut(0).unwrap().insert(d1.cards.clone());
        }
        if player_history[1].contains(&d1.cards) {

            println!("Player 1 wins round {} in game {} on infinite recursion", round, game);
            if *verbose {
                println!("{}'s deck {}", d2.player, d2.cards_to_string(&0));
            }
            return 0;
        } else {
            player_history.get_mut(0).unwrap().insert(d2.cards.clone());
        }
        match d1.get_card() {
            Some(player_one) => {
                match d2.get_card() {
                    Some(player_two) => {
                        if *verbose {
                            println!("\n--Round {} (Game {}) --", round, game);
                            println!("{}'s deck {}", d1.player, d1.cards_to_string(&player_one));
                            println!("{}'s deck {}", d2.player, d2.cards_to_string(&player_two));
                            println!("{} plays: {}", d1.player, player_one);
                            println!("{} plays: {}", d2.player, player_two);
                        }
                        if d1.cards() + 1 > player_one && d2.cards() + 1 > player_two {
                            if *verbose {
                                println!("Playing a sub-game to determine the winner...");
                            }
                            match play_recursive(
                                d1.trim_deck_clone(&player_one),
                                d2.trim_deck_clone(&player_two),
                                game + 1,
                                verbose,
                            ) {
                                0 => {
                                    if *verbose {
                                        println!("\n...anyway, back to game {}", game);
                                        println!("{} wins round {} of game {}!", d1.player, round, game);
                                    }
                                    d1.gain_cards(player_one, player_two);
                                }
                                1 => {
                                    if *verbose {
                                        println!("\n...anyway, back to game {}", game);
                                        println!("{} wins round {} of game {}!", d2.player, round, game);
                                    }
                                    d2.gain_cards(player_two, player_one);
                                }
                                _ => panic!("Unknown winner of game!")
                            }
                        } else if player_one > player_two {
                            if *verbose {
                                println!("{} wins round {} of game {}!", d1.player, round, game);
                            }
                            d1.gain_cards(player_one, player_two);
                        } else {
                            if *verbose {
                                println!("{} wins round {} of game {}!", d2.player, round, game);
                            }
                            d2.gain_cards(player_two, player_one);
                        }
                    },
                    None => {
                        d1.return_card(player_one);
                        break;
                    }
                }
            },
            None => {
                break;
            }        
        }
        round += 1;     
    }

    if *verbose || game == 1 {
        println!("{} scores: {}", d1.player, d1.score());
        println!("{} scores: {}", d2.player, d2.score());
    }
    match d1.cards() > 0 { true => 0, false => 1 }
}

fn main() {
    let verbose = false;
    let is_demo = false;
    let is_demo2 = false;
    let data = match is_demo {
        true => match is_demo2 { true => load_demo2(), false => load_demo()},
        false => load_data(),
    };
    let decks = Deck::from_string(data);
    if !is_demo || !is_demo2 {
        play(decks.clone());
    }
    play_recursive(decks[0].clone(), decks[1].clone(), 1, &verbose);
}
