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
enum HandRanking {
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
    impl Evaluate{
        pub fn Evaluate(community_cards: vec![]){
            let mut card1 = self.0
            let mut card2 = self.1
            let playable_cards = vec![card1,card2,community_cards];
            let mut x = 0
            let mut heart_count = 0
            let mut heart_cards = vec![]
            let mut club_count = 0
            let mut club_cards = vec![]
            let mut spade_count = 0
            let mut spade_cards = vec![]
            let mut diamond_count = 0
            let mut diamond_cards = vec![]
            let mut rank_vec = vec![1...7]

            for playable_cards[x] in playable_cards.len(){
                if playable_cards[x].suit == 'h'{
                    heart_cards[heart_count] = playable_cards[x].rank;
                    heart_count += 1;
                }
                else if playable_cards[x].suit == 'c'{
                    club_cards[club_count] = playable_cards[x].rank;
                    club_count += 1;
                }
                else if playable_cards[x].suit == 's'{
                    spade_cards[spade_count] = playable_cards[x].rank;
                    spade_count += 1;
                }
                else if playable_cards[x].suit == 'd'{
                    diamond_cards[diamond_count] = playable_cards[x].rank;
                    diamond_count += 1
                }
                rank_vec[x] = playable_cards[x].rank
                x +=1
            }

            let 2_count = rank_vec.iter().filter(|&n| *n == 2).count())
            if 2_count = 2 {
                let mut 2_output == 'Pair_of_Twos'
            }
            else if 2_count == 3 {
                let mut 2_output = 'Triple_Twos'
            }
            else if 2_count == 4 {
                let mut 2_output = 'Quad_Twos'
            }
            let 3_count = rank_vec.iter().filter(|&n| *n == 3).count())
            if 3_count == 1 {
                let mut 3_output = 'Three_High'
            }
            else if 3_count == 2 {
                let mut 3_output = 'Pair_of_Threes'
            }
            else if 3_count == 3 {
                let mut 3_output = 'Triple_Threes'
            }
            else if 3_count == 4 {
                let mut 3_output = 'Quad_Threes'
            }
            let 4_count = rank_vec.iter().filter(|&n| *n == 4).count())
            if 4_count == 1 {
                let mut 4_output = 'Four_High'
            }
            else if 4_count = 2 {
                let mut 4_output = 'Pair_of_Fours'
            }
            else if 4_count = 3 {
                let mut 4_output = 'Triple_Fours'
            }
            else if 4_count = 4 {
                let mut 4_output = 'Quad_Fours'
            }
            let 5_count = rank_vec.iter().filter(|&n| *n == 5).count())
            if 5_count == 1 {
                let mut 5_output = 'Five_High'
            }
            else if 5_count == 2 {
                let mut 5_output = 'Pair_of_Fives'
            }
            else if 5_count == 3 {
                let mut 5_output = 'Triple_Fives'
            }
            else if 5_count == 4 {
                let mut 5_output = 'Quad_Fives'
            let 6_count = rank_vec.iter().filter(|&n| *n == 6).count())
            if 6_count == 1 {
                let mut 6_output = 'Six_High'
            }
            else if 6_count == 2 {
                let mut 6_output = 'Pair_of_Sixes'
            }
            else if 6_count == 3 {
                let mut 6_output = 'Triple_Sixes'
            }
            else if 6_count == 4 {
                let mut 6_output = 'Quad_Sixes'
            let 7_count = rank_vec.iter().filter(|&n| *n == 7).count())
            if 7_count == 1 {
                let mut 7_output = 'Seven_High'
            }
            else if 7_count == 2 {
                let mut 7_output = 'Pair_of_Sevens'
            }
            else if 7_count == 3 {
                let mut 7_output = 'Triple_Sevens'
            }
            else if 7_count == 4 {
                let mut 7_output = 'Quad_Sevens'
            let 8_count = rank_vec.iter().filter(|&n| *n == 8).count())
            if 8_count == 1 {
                let mut 8_output = 'Eight_High'
            }
            else if 8_count == 2 {
                let mut 8_output = 'Pair_of_Eights'
            }
            else if 8_count == 3 {
                let mut 8_output = 'Triple_Eights'
            }
            else if 8_count == 4 {
                let mut 8_output = 'Quad_Eights'
            let 9_count = rank_vec.iter().filter(|&n| *n == 9).count())
            if 9_count == 1 {
                let mut 9_output = 'Nine_High'
            }
            else if 9_count == 2 {
                let mut 9_output = 'Pair_of_Nines'
            }
            else if 9_count == 3 {
                let mut 9_output = 'Triple_Nines'
            }
            else if 9_count == 4 {
                let mut 9_output = 'Quad_Nines'
            let 10_count = rank_vec.iter().filter(|&n| *n == 10).count())
            if 10_count == 1 {
                let mut 10_output = 'Ten_High'
            }
            else if 10_count == 2 {
                let mut 10_output = 'Pair_of_Tens'
            }
            else if 10_count == 3 {
                let mut 10_output = 'Triple_Tens'
            }
            else if 10_count == 4 {
                let mut 10_output = 'Quad_Tens'
            let j_count = rank_vec.iter().filter(|&n| *n == 11).count())
            if j_count == 1 {
                let mut j_output = 'Jack_High'
            }
            else if j_count == 2 {
                let mut j_output = 'Pair_of_Jacks'
            }
            else if j_count == 3 {
                let mut j_output = 'Triple_Jacks'
            }
            else if j_count == 4 {
                let mut j_output = 'Quad_Jacks'
            let q_count = rank_vec.iter().filter(|&n| *n == 12).count())
            if q_count == 1 {
                let mut q_output = 'Queen_High'
            }
            else if q_count == 2 {
                let mut q_output = 'Pair_of_Queens'
            }
            else if q_count == 3 {
                let mut q_output = 'Triple_Queens'
            }
            else if q_count == 4 {
                let mut q_output = 'Quad_Queens'
            let k_count = rank_vec.iter().filter(|&n| *n == 13).count())
            if 3_count == 1 {
                let mut k_output = 'King_High'
            }
            else if k_count == 2 {
                let mut k_output = 'Pair_of_Kings'
            }
            else if k_count == 3 {
                let mut k_output = 'Triple_Kings'
            }
            else if k_count == 4 {
                let mut k_output = 'Quad_Kings'
            let a_count = rank_vec.iter().filter(|&n| *n == 14).count())
            if a_count == 1 {
                let mut a_output = 'Ace_High'
            }
            else if a_count == 2 {
                let mut a_output = 'Pair_of_Aces'
            }
            else if a_count == 3 {
                let mut a_output = 'Triple_Aces'
            }
            else if a_count == 4 {
                let mut a_output = 'Quad_Aces'
            }
            if heart_count >= 5 {
                let flush_cards = heart_cards; 
            }
            else if club_count >= 5 {
                let flush_cards = club_cards;
            }
            else if spade_count >= 5 {
                let flush_cards = spade_cards;
            }
            else if diamond_count >= 5 {
                let flush_cards = diamond_cards
            }
            if (heart_cards.contains(14) && heart_cards.contains(13) && heart_cards.contains(12) && heart_cards.contains(11) && heart_cards.contains(10)) || (club_cards.contains(14) && club_cards.contains(13) && club_cards.contains(12) && club_cards.contains(11) && club_cards.contains(10)) || (spade_cards.contains(14) && spade_cards.contains(13) && spade_cards.contains(12) && spade_cards.contains(11) && spade_cards.contains(10)) || (diamond_cards.contains(14) && diamond_cards.contains(13) && diamond_cards.contains(12) && diamond_cards.contains(11) && diamond_cards.contains(10)){
                let hand_ranking = Royal_Flush
            }
            else if (flush_cards.contains(14)  && flush_cards.contains(2) && flush_cards.contains(3) && flush_cards.contains(4) && flush_cards.contains(5)) || (flush_cards.contains(2)  && flush_cards.contains(3) && flush_cards.contains(4) && flush_cards.contains(5) && flush_cards.contains(6)) || (flush_cards.contains(3)  && flush_cards.contains(4) && flush_cards.contains(5) && flush_cards.contains(6) && flush_cards.contains(7)) || (flush_cards.contains(4)  && flush_cards.contains(5) && flush_cards.contains(6) && flush_cards.contains(7) && flush_cards.contains(8)) || (flush_cards.contains(5)  && flush_cards.contains(6) && flush_cards.contains(7) && flush_cards.contains(8) && flush_cards.contains(9)) || (flush_cards.contains(6)  && flush_cards.contains(7) && flush_cards.contains(8) && flush_cards.contains(9) && flush_cards.contains(10)) (flush_cards.contains(7)  && flush_cards.contains(8) && flush_cards.contains(9) && flush_cards.contains(10) && flush_cards.contains(11)) || (flush_cards.contains(8)  && flush_cards.contains(9) && flush_cards.contains(10) && flush_cards.contains(11) && flush_cards.contains(12)) || (flush_cards.contains(9)  && flush_cards.contains(10) && flush_cards.contains(11) && flush_cards.contains(12) && flush_cards.contains(13))
            else if flush_cards == true{
             let hand_ranking = Flush   
            }
            else if (rank_vec.contains(14)  && rank_vec.contains(2) && rank_vec.contains(3) && rank_vec.contains(4) && rank_vec.contains(5)) || (rank_vec.contains(2)  && rank_vec.contains(3) && rank_vec.contains(4) && rank_vec.contains(5) && rank_vec.contains(6)) || (rank_vec.contains(3)  && rank_vec.contains(4) && rank_vec.contains(5) && rank_vec.contains(6) && rank_vec.contains(7)) || (rank_vec.contains(4)  && rank_vec.contains(5) && rank_vec.contains(6) && rank_vec.contains(7) && rank_vec.contains(8)) || (rank_vec.contains(5)  && rank_vec.contains(6) && rank_vec.contains(7) && rank_vec.contains(8) && rank_vec.contains(9)) || (rank_vec.contains(6)  && rank_vec.contains(7) && rank_vec.contains(8) && rank_vec.contains(9) && rank_vec.contains(10)) (rank_vec.contains(7)  && rank_vec.contains(8) && rank_vec.contains(9) && rank_vec.contains(10) && rank_vec.contains(11)) || (rank_vec.contains(8)  && rank_vec.contains(9) && rank_vec.contains(10) && rank_vec.contains(11) && rank_vec.contains(12)) || (rank_vec.contains(9)  && rank_vec.contains(10) && rank_vec.contains(11) && rank_vec.contains(12) && rank_vec.contains(13)) || (rank_vec.contains(10)  && rank_vec.contains(11) && rank_vec.contains(12) && rank_vec.contains(13) && rank_vec.contains(14)) {
                let hand_ranking = Straight
            }
            // else if a_count == 3 || k_count == 3 || q_count == 3 || j_count == 3 || 10_count == 3 || 9_count == 3 || 8_count == 3 || 7_count == 3 || 6_count == 3 || 5_count == 3 || 4_count == 3 || 3_count == 3 || 2_count == 3) && (a_count == 2 || k_count == 2 || q_count == 2 || j_count == 2 || 10_count == 2 || 9_count == 2 || 8_count == 2 || 7_count == 2 || 6_count == 2 || 5_count == 2 || 4_count == 2 || 3_count == 2 || 2_count == 2) 
                     
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