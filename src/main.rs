use pokersite::deck::*;

fn main() {
    let test_deck = Deck::new();
    println!("There are {:?} cards in the deck", test_deck.cards.len());

}
