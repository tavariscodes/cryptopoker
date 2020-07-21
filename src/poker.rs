pub mod deck;
pub mod table;
pub mod player;


// Test poker modules
#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn create_table() {
        let table_id= String::from("721-11");
        let table = table::Table::new(table_id, 0.10, 0.20, 5.00, 20.00, 6);
        assert_eq!(table.id, "721-11");
        assert_eq!(table.small_blind, 0.10);
        assert_eq!(table.big_blind, 0.20);
        assert_eq!(table.min_buyin, 5.00);
        assert_eq!(table.max_buyin, 20.00);
        assert_eq!(table.seat_count, 6);
    }

    #[test]
    fn create_player() {
        let mut player = player::Player::new(String::from("Tavaris"), 60.0, true);
        
        assert_eq!(player.name, "Tavaris");
        assert_eq!(player.chips_in_play, 60.0);
        assert_eq!(player.sitting_in, true);
    }

    #[test]
    fn test_player_join() {
        let table_id= String::from("721-11");
        let mut table = table::Table::new(table_id, 0.10, 0.20, 5.00, 20.00, 6);
        let mut test_player = player::Player::new(String::from("Tavaris"), 60.0, true);
        
        table.player_joined(test_player, 2, 25.00);
        table.start_round();

        assert_eq!(table.seats.len(), 1);
    }

    #[test]
    fn create_deck() {
        // ensure 52 cards are in deck.
        let mut deck = deck::Deck::new();
        assert_eq!(52, deck.cards.len());
    }

    #[test]
    fn shuffle() {
        // test for proper cards being in deck

    }
}