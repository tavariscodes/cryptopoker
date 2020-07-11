//! # Table
//! 
//! 'Table' a library  
//! implementing the needed functionality
//! for creating a poker table
//! 


use crate::poker::deck;

/// Pot 
struct Pot {
    amount: f64,
    contributers: Vec<String>       // change to vec of type "Players"
}

/// Creates a new pot, with amount set to 0
impl Pot {
    pub fn new() -> Pot {
        Pot {
            amount: 0.00,
            contributers: vec![],
        }
    }
}
/// Struct to model a poker table
pub struct Table {
    pub id: String,
    pub small_blind: f64,
    pub big_blind: f64,
    pub min_buyin: f64,
    pub max_buyin: f64,
    pub seat_count: u8,
    community_cards: [Option<deck::Card>; 5],
    deck: deck::Deck,
    pot: Pot,
    players_sitting: u8,
    players_in_hand: u8,
    betting_round: Option<String>,
}   
impl Table {
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
            community_cards: [None, None, None, None, None],
            pot: Pot::new(),
            players_sitting: players_sitting,
            players_in_hand: players_in_hand,
            betting_round: None   // change to type 'BettingRound::preflop'
        }
    }
    /// Starts a new round
    fn start_round(&mut self) {
        self.deck.shuffle();    
        self.players_in_hand = 0;

        // check to see if proper # of players
        // deal cards
        // post blinds
    }
    
}
