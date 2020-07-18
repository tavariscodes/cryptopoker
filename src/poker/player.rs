//! # Player
//! 
//! `Player` a library modeling the actions 
//! of a poker player containing
//! the neccessary functionality 
//! to interact with the poker table
//! 

use crate::poker::deck;

/// A 2 card poker hand
#[derive(Debug)]
pub struct Hand(deck::Card, deck::Card);

/// Struct modeling an individual poker player
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub chips_in_play: f64,
    pub sitting_in: bool,
    pub in_hand: bool,
    pub has_cards: bool,
    //pub hand: Hand,
    bet: f64,
    chips: f64,                     
    room: Option<String>,           // consider changing type to `table` (events) instead
    sitting_on_table: Option<String>,       // table id 
    pub seat: Option<usize>,                       // seat number player is sitting
    cards: Option<Hand> ,           // players card
    evaluated_hand: String          // winning hand type
}

impl Player {
    /// Create new player
    pub fn new(name: String, chips_in_play: f64,
    sitting_in: bool) -> Player {
        Player {
            name: name,
            chips_in_play: chips_in_play,
            sitting_in: sitting_in,
            in_hand: false,
            has_cards: false,
            bet: 0.0,
            chips: 0.0,
            room: None,
            sitting_on_table: None,
            seat: None,
            cards: None,
            evaluated_hand: String::from("N/a")     // change to type HandRank
        }
    }

    /// Adds chips to player's account
    fn deposit(&mut self, amount: f64) {
        self.chips += amount;
    }

    /// initiliaze a new round, by resetting players
    ///  public props
    pub fn prepare_new_round(&mut self) {
        self.cards = None;
        self.has_cards = false;
        self.bet = 0.0;
        self.in_hand = true;
        self.evaluated_hand = String::from("");
    }
    
    /// Sits player onto table
    pub fn sit_on_table(&mut self, table_id: String, seat: u8, chips: f64) {
        // remove # of chips player buys in
        // on table
        self.chips -= chips;
        self.chips_in_play = chips;
        self.seat = Some(seat);
        self.sitting_on_table = Some(table_id);   
        // send event to table 
        // that player has sat down
    }

    /// Sits player out from 
    /// table
    pub fn sit_out(&mut self) {
        match &self.sitting_on_table {
            None => panic!("Error: player not sitting on table"),
            Some(_table) => {
                self.in_hand = false;
                self.sitting_in = false;
            }
        }
    }

    /// Exits player from table
    pub fn leave_table(&mut self) {
        match &self.sitting_on_table {
            None => panic!("Error: player not sitting on table"),
            Some(_table) => {
                self.sit_out();
                // add chips from table 
                // into player's bankroll
                self.chips += self.chips_in_play;
                self.chips_in_play = 0.0;           
                self.sitting_on_table = None;
                self.seat = None;
            }
        }
    }
    
    /// The action of betting
    pub fn bet(&mut self, mut amount: f64) {
        if amount > self.chips_in_play {
            amount = self.chips_in_play;
        }
        self.chips_in_play -= amount;
        self.bet += amount 
    }

    /// The action of raising
    pub fn raise(&mut self, amount: f64) {
        self.bet(amount);
    }

    /// The action of folding a hand
    pub fn fold (&mut self) {
        self.cards = None;
        self.has_cards = false;
        self.in_hand = false;
    }
}