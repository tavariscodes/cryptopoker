//! # Table
//! 
//! 'Table' a library  
//! implementing the needed functionality
//! for creating a poker table
//! 


pub mod table {
    use crate::deck::deck;

    struct Pot {
        amount: f64,
        contributers: Vec<String>       // change to vec of type "Players"
    }

    impl Pot {
        pub fn new() -> Pot {
            Pot {
                amount: 0.00,
                contributers: vec![],
            }
        }
    }

   pub struct Table {
       pub id: String,
       pub small_blind: f64,
       pub big_blind: f64,
       pub min_buyin: f64,
       pub max_buyin: f64,
       pub seat_count: u8,
       deck: deck::Deck,
       pot: Pot,
       players_sitting: u8,
       players_in_hand: u8,
       betting_round: String,
    }   

    /// Create a new table 
    pub fn new(id: String, small_blind: f64, big_blind: f64, min_buyin: f64,  max_buyin: f64,
    seat_count: u8, players_sitting: u8, players_in_hand: u8) -> Table {
       Table {
           id: id,
           small_blind: small_blind,
           big_blind: big_blind,
           min_buyin: min_buyin,
           max_buyin: max_buyin,
           seat_count: seat_count,
           deck: deck::Deck::new(),
           pot: Pot::new(),
           players_sitting: players_sitting,
           players_in_hand: players_in_hand,
           betting_round: String::from("preflop")   // change to type 'BettingRound::preflop'
       }
    }

    /// Starts a new game
    fn start_game() {

    }
}