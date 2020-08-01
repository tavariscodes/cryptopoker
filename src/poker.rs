pub mod deck;
pub mod table;
pub mod player;

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn create_table() {
        let table_id= String::from("721-11");
        let table = table::Table::new(table_id, 0.10, 0.20, 5.00, 20.00, 6);
        assert_eq!(table.id, "721-11");
    }
    #[test]
    fn test_player_join() {
        let table_id= String::from("721-11");
        let mut table = table::Table::new(table_id, 0.10, 0.20, 5.00, 20.00, 6);
        let mut player = player::Player::new(String::from("Tavaris"), 60.0, true);
        
        table.player_joined(player,3,50.00);
        table.start_round();
        assert_eq!(table.seats.len(), 1);
    }

    #[test]
    fn test_evaluate(){   
        let mut card1 = deck::Card::new(deck::Rank::Ace, deck::Suit::Spade);
        let mut card2 = deck::Card::new(deck::Rank::Queen, deck::Suit::Spade);
         
        let mut player_hand = player::Hand(card1, card2);

        let mut card3 = deck::Card::new(deck::Rank::Six, deck::Suit::Spade);
        let mut card4 = deck::Card::new(deck::Rank::Eight, deck::Suit::Spade);
        let mut card5 = deck::Card::new(deck::Rank::Two, deck::Suit::Spade);
        let mut card6 = deck::Card::new(deck::Rank::Ace, deck::Suit::Diamond);
        let mut card7 = deck::Card::new(deck::Rank::Ace, deck::Suit::Club);
        let mut community_cards = [Some(card3), Some(card4), Some(card5), Some(card6), Some(card7)];
        let cooter =player_hand.Evaluate(&community_cards);
        assert_eq!(cooter, 6)
    }
    
    fn test_shuffle() {
        // ensure all 52 cards are in deck.
        
    }
}