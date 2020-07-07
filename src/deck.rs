//! # Deck
//! 
//! `Deck` a library
//! containing the neccessary functionality
//! for a proper deck implementation.

pub mod deck {
    /// The four different card suits 
    #[derive(Debug)]
    pub enum Suit {
        Club,
        Heart,
        Spade,
        Diamond
    }

    impl Suit {
        /// Returns a suit's corresponding character
        pub fn to_char(&self) -> char {
            match self {
                Suit::Club => 'c',
                Suit::Heart => 'h',
                Suit::Spade => 's',
                Suit::Diamond => 'd',
                _ => (panic!("Error: item not of type - `Suit`"))
            }
        }
    }
    
    #[derive(Debug, PartialEq, Copy, Clone)]
    /// The fourteen different card rankings
    pub enum Rank {
        Two = 2,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace
    }

    impl Rank {
        pub fn get_rank(val: u8) -> Rank {
            match val {
                2 => Rank::Two ,
                3 => Rank::Three,
                4 => Rank::Four,
                5 => Rank::Five,
                6 => Rank::Six,
                7 => Rank::Seven,
                8 => Rank::Eight,
                9 => Rank::Nine,
                10 => Rank::Ten,
                11 => Rank::Jack,
                12 => Rank::Queen,
                13 => Rank::King,
                14 => Rank::Ace,
                _ => (panic!("Error: INVALID INPUT | valid ranking values are 2-14"))
            }
        }
    }

 

    /// Represents a card which contains 
    /// a rank and a suit
    #[derive(Debug)]
    pub struct Card {
        pub rank: Rank,
        pub suit: Suit
    }

    impl Card {
        /// Creates a card given a rank and suit 
        pub fn new(rank: Rank, suit: Suit) -> Card {
            Card {
                rank, 
                suit
            }
        }
    }

    /// Deck containing 52 playing cards
    #[derive(Debug)]
    pub struct Deck {
        pub cards: Vec<Card>
    }

    impl Deck {
        /// Creates a new deck with 52 cards
        pub fn new() -> Deck {
            let mut deck_of_cards: Vec<Card> = vec![];  
            for val in 2..15 {
                // get each card's rank,
                // and assign it to a suit 
                let card_rank = Rank::get_rank(val);
                // add card to deck
                deck_of_cards.push(Card::new(card_rank, Suit::Club));
                deck_of_cards.push(Card::new(card_rank, Suit::Heart));
                deck_of_cards.push(Card::new(card_rank, Suit::Spade));
                deck_of_cards.push(Card::new(card_rank, Suit::Diamond));
            }
            Deck {
                cards: deck_of_cards,
            }
        }

        pub fn deal() {}

        fn shuffle() {
            // shuffle deck
        }
    }

}






