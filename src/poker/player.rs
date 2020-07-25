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
pub enum HandRanking {
    HighCard = 1,
    Pair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
    RoyalFlush = 10
}


    impl Hand{
        pub fn Evaluate(&self,community_cards: [deck::Card])->u8{
            let mut card1 = self.0;
            let mut card2 = self.1;
            let playable_cards = vec![card1,card2,community_cards];      
            let mut heart_count = 0;
            let mut heart_cards = vec![];
            let mut club_count = 0;
            let mut club_cards = vec![];
            let mut spade_count = 0;
            let mut spade_cards = vec![];
            let mut diamond_count = 0;
            let mut diamond_cards = vec![];
            let mut rank_vec = vec![];
            let mut flush_exists = false;
            let mut flush_cards = vec![];
            let mut x = 0;
            let mut card_suit0 = playable_cards[0].suit.to_char();
            let mut card_suit1 = playable_cards[1].suit.to_char();
            let mut card_suit2 = playable_cards[2].suit.to_char();
            let mut card_suit3 = playable_cards[3].suit.to_char();
            let mut card_suit4 = playable_cards[4].suit.to_char();
            let mut card_suit5 = playable_cards[5].suit.to_char();
            let mut card_suit6 = playable_cards[6].suit.to_char();
            let mut card_suits = [card_suit0, card_suit1, card_suit2, card_suit3,card_suit4, card_suit5, card_suit6];
            for x in 0..6{
                if card_suits[x] == 'h'{
                    heart_cards[heart_count] = playable_cards[x].rank;
                    heart_count += 1;
                }
                else if card_suits[x] == 'c'{
                    club_cards[club_count] = playable_cards[x].rank;
                    club_count += 1;
                }
                else if card_suits[x] == 's'{
                    spade_cards[spade_count] = playable_cards[x].rank;
                    spade_count += 1;
                }
                else if card_suits[x] == 'd'{
                    diamond_cards[diamond_count] = playable_cards[x].rank;
                    diamond_count += 1;
                }
                rank_vec[x] = playable_cards[x].rank;
            }
            // need a way to find mode of vector for now variable will be called mode_of_cards
            if heart_count >= 5 {
                let flush_cards = heart_cards;
                flush_exists = true; 
            }
            else if club_count >= 5 {
                let flush_cards = club_cards;
                flush_exists = true;
            }
            else if spade_count >= 5 {
                let flush_cards = spade_cards;
                flush_exists = true;
            }
            else if diamond_count >= 5 {
                let flush_cards = diamond_cards;
                flush_exists = true;
            }
            //HAND EVALUATION
                //FLUSHES
            if flush_exists == true{
                if flush_cards.contains(&14) && flush_cards.contains(&13) && flush_cards.contains(&12) && flush_cards.contains(&11) && flush_cards.contains(&10) == true {
                    return 10;
                }
                else if (flush_cards.contains(&14)  && flush_cards.contains(&2) && flush_cards.contains(&3) && flush_cards.contains(&4) && flush_cards.contains(&5)) || (flush_cards.contains(&2)  && flush_cards.contains(&3) && flush_cards.contains(&4) && flush_cards.contains(&5) && flush_cards.contains(&6)) || (flush_cards.contains(&3)  && flush_cards.contains(&4) && flush_cards.contains(&5) && flush_cards.contains(&6) && flush_cards.contains(&7)) || (flush_cards.contains(&4)  && flush_cards.contains(&5) && flush_cards.contains(&6) && flush_cards.contains(&7) && flush_cards.contains(&8)) || (flush_cards.contains(&5)  && flush_cards.contains(&6) && flush_cards.contains(&7) && flush_cards.contains(&8) && flush_cards.contains(&9)) || (flush_cards.contains(&6)  && flush_cards.contains(&7) && flush_cards.contains(&8) && flush_cards.contains(&9) && flush_cards.contains(&10)) || (flush_cards.contains(&7)  && flush_cards.contains(&8) && flush_cards.contains(&9) && flush_cards.contains(&10) && flush_cards.contains(&11)) || (flush_cards.contains(&8)  && flush_cards.contains(&9) && flush_cards.contains(&10) && flush_cards.contains(&11) && flush_cards.contains(&12)) || (flush_cards.contains(&9)  && flush_cards.contains(&10) && flush_cards.contains(&11) && flush_cards.contains(&12) && flush_cards.contains(&13))== true{
                    return 9;
                }
                else{
                    return 6;    
                }
            }
                //STRAIGHTS
            else if rank_vec.contains(&deck::Rank::Ace) && rank_vec.contains(&deck::Rank::King) && rank_vec.contains(&deck::Rank::Queen) && rank_vec.contains(&deck::Rank::Jack) && rank_vec.contains(&deck::Rank::Ten){
                    return 5;  
            }
                //HIGH CARD HANDS
            else{
                let number1 = rank_vec[1];
                let number3 = rank_vec[3];
                let number5 = rank_vec[5];
                let mut mode_of_cards_count = 0;
                let mut nd_mode_of_cards_count = 0;
            let cards_count1 = rank_vec.iter().filter(|&n| *n == number1).count();
            let cards_count3 = rank_vec.iter().filter(|&n| *n == number3).count();
            let cards_count5 = rank_vec.iter().filter(|&n| *n == number5).count();
            if (cards_count1 >= cards_count3) & (cards_count1 >= cards_count5) & (cards_count3 >= cards_count5){
                 mode_of_cards_count = cards_count1;
                let mut mode_of_cards = number1;
                 nd_mode_of_cards_count = cards_count3;
                let mut nd_mode_of_cards = number3;
            }
            else if (cards_count3 > cards_count1) & (cards_count1 >= cards_count5) {
                 mode_of_cards_count = cards_count3;
                let mut mode_of_cards = number3;
                 nd_mode_of_cards_count = cards_count1;
                let mut nd_mode_of_cards = number1;
            }
            else if (cards_count5 > cards_count1) & (cards_count1 >= cards_count3) {
                 mode_of_cards_count = cards_count5;
                let mut mode_of_cards = number5;
                 nd_mode_of_cards_count = cards_count1;
                let mut nd_mode_of_cards = number1;
            }
            else if (cards_count5 > cards_count1) & (cards_count3 > cards_count1) & (cards_count3 >= cards_count5){
                 mode_of_cards_count = cards_count3;
                let mut mode_of_cards = number3;
                 nd_mode_of_cards_count = cards_count5;
                let mut nd_mode_of_cards = number5;
            }
            else if (cards_count1 >= cards_count3) & (cards_count1 >= cards_count5) & (cards_count5 > cards_count3){
                 mode_of_cards_count = cards_count1;
                let mut mode_of_cards = number1;
                 nd_mode_of_cards_count = cards_count5;
                let mut nd_mode_of_cards = number5;
            }
            else {
                 mode_of_cards_count = cards_count5;
                let mut mode_of_cards = number5;
                 nd_mode_of_cards_count = cards_count3;
                let mut nd_mode_of_cards = number3;
            }
                if mode_of_cards_count == 1{
                    return 1;
                }
                else if mode_of_cards_count == 2 && nd_mode_of_cards_count == 1{
                    return 2;
                }
                else if mode_of_cards_count == 2 && (nd_mode_of_cards_count == 2 || nd_mode_of_cards_count == 3) {
                    return 3;
                }
                else if mode_of_cards_count == 3 && nd_mode_of_cards_count == 1 {
                    return 4;
                }
                else if mode_of_cards_count == 3 && nd_mode_of_cards_count == 2 {
                    return 7;
                }
                else {
                    return 8;
                }              
            }
        }
    }

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