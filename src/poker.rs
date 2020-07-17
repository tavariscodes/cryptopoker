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
    fn test_find_active_seat(){
        let table_id= String::from("721-11");
        let mut player1 = player::Player::new(String::from("Tavaris"), 60.0, true);
        let mut player2 = player::Player::new(String::from("Whoops"), 60.0, true);
        let mut player3 = player::Player::new(String::from("Jack"), 60.0, true);
        let mut table = table::Table::new(table_id, 0.10, 0.20, 5.00, 20.00, 6);
        table.player_joined(player1,3,20.00);
        table.player_joined(player2,4,20.00);
        table.player_joined(player3,6,20.00);
        table.find_active_seat();
        assert_eq!(table.find_active_seat(), 3);
    }
    
    fn test_shuffle() {
        // ensure all 52 cards are in deck.
        
    }
}