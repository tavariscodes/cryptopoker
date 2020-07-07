use pokersite::deck::*;

fn main() {
    let test_deck = Deck::new();
    assert_eq!(52, test_deck.cards.len());
}
