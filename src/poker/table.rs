//! # Table
//! 
//! `Table` a library which
//! implements the needed functionality
//! for creating a poker table
//! 
use crate::poker::deck;
use crate::poker::player;

/// Pot 
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Table {
    pub id: String,
    pub small_blind: f64,
    pub big_blind: f64,
    pub min_buyin: f64,
    pub max_buyin: f64,
    pub seat_count: usize,
    pub active_seat: usize,    /// seat # of the player whose turn it is
    pub dealer_seat: usize,
    community_cards: [Option<deck::Card>; 5],
    pub seats: Vec<player::Player>,     // make private
    deck: deck::Deck,
    pot: Pot,
    pub players_seated: usize, /// # of players sitting at table
    players_sitting_in: usize,
    players_in_hand: usize,
    betting_round: Option<BettingRound>,
}   

impl Table {
    /// Create a new table 
    pub fn new(id: String, small_blind: f64, big_blind: f64, min_buyin: f64,  max_buyin: f64,
        seat_count: usize) -> Table {
        Table {
            id: id,
            small_blind: small_blind,
            big_blind: big_blind,
            min_buyin: min_buyin,
            max_buyin: max_buyin,
            active_seat: 0,
            seat_count: seat_count,
            seats: vec![],
            dealer_seat: 0,
            deck: deck::Deck::new(),
            community_cards: [None, None, None, None, None],
            pot: Pot::new(),
            players_seated: 0,
            players_sitting_in: 0,
            players_in_hand: 0,
            betting_round: None
        }
    }
    
    /// finds next active player at table
    fn find_next_player(&mut self, offset: usize, status: String) -> usize {
        // use offset value to find
        // next active player from 
        // offset. 
        
    }

    /// Starts a new round
    pub fn start_round(&mut self) {
        if self.players_seated > 1 {
            self.deck.shuffle();    
            self.betting_round = Some(BettingRound::Preflop);
            for player in self.seats.iter_mut() {
                player.prepare_new_round();
                // find next active player and 
                // make them the dealer
            }
        };
        // deal cards
        // post blinds
}

    /// Adds player to table 
    pub fn player_joined(&mut self, player: player::Player, seat: u8, chips: f64) {
        self.seats.push(player);
        self.players_seated += 1;
    }
}

#[derive(Debug)]
enum BettingRound {
    Preflop,
    Flop,
    Turn,
    River,
    Showdown
}

impl BettingRound {
    /// initalizes a particular betting round
    fn initialize(&mut self, round: BettingRound) -> () {
        // action to next player
        match round {
            BettingRound::Preflop => self.preflop(),
            BettingRound::Flop => self.flop(),
            BettingRound::Turn => self.turn(),
            BettingRound::River => self.river(),
            BettingRound::Showdown => self.showdown(),
            _ => panic!("Error: Method requires betting round to be specfied.")
        }
    }

    fn preflop(&self) {}

    fn flop(&self) {}

    fn turn(&self) {}
    
    fn river(&self) {}
     
    fn showdown(&self) {}
    
}