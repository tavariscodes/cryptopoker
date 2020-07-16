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
        
        table.player_joined(player);
        table.start_round();
        assert_eq!(table.seats.len(), 1);
    }

    fn test_shuffle() {
        // ensure all 52 cards are in deck.
        
    }
}