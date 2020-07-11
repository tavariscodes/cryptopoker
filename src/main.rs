use pokersite::poker::deck::*;

fn main() {
    let mut test_deck = Deck::new();
    assert_eq!(52, test_deck.cards.len());

    let test_suit = Suit::Club;
    assert_eq!('c', test_suit.to_char());

    test_deck.shuffle();
}
