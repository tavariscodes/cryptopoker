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
    pub seat_count: u8,
    pub active_seat: u8,    /// seat # of the player whose turn it is
    community_cards: [Option<deck::Card>; 5],
    pub seats: Vec<player::Player>,     // make private
    deck: deck::Deck,
    pub dealer_seat: usize,    //  gets vector of seats
    pot: Pot,
    players_sitting: u8,
    players_in_hand: u8,
    betting_round: Option<BettingRound>,
}   

impl Table {
    /// Create a new table 
    pub fn new(id: String, small_blind: f64, big_blind: f64, min_buyin: f64,  max_buyin: f64,
        seat_count: u8) -> Table {
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
            players_sitting: 0,
            players_in_hand: 0,
            betting_round: None
        }
    }
    // add method to increase # of players_sitting
    // when player sits down. 

    

    /// Starts a new round
    pub fn start_round(&mut self) {
        if self.players_sitting > 1 {
            self.deck.shuffle();
            self.dealer_seat += 1;
            while self.seats[self.dealer_seat].in_hand == false {
                self.dealer_seat += 1;
            }
            let mut small_blind = self.dealer_seat + 1;
            while self.seats[small_blind].in_hand == false{
                small_blind += 1;
            }
            let mut big_blind = small_blind + 1;
            while self.seats[big_blind].in_hand == false{
                big_blind += 1;
            }
            self.betting_round = Some(BettingRound::Preflop);
            for player in self.seats.iter_mut() {
                player.prepare_new_round();
            }
        };
        // deal cards
        // post blinds
    }

    /// Adds player to table 
    pub fn player_joined(&mut self, player: player::Player, seat: u8, chips: f64) {
        self.seats.push(player);
        self.players_sitting += 1;
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

    fn preflop(&self) {     
    }

    fn flop(&self) {}

    fn turn(&self) {}
    
    fn river(&self) {}
     
    fn showdown(&self) {}
    
}