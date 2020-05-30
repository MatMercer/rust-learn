fn main() {
    let deck = build_poker_deck(true);
    for c in build_poker_deck(true) {
        render_card(&c);
    }

    println!("Generated {} cards.", deck.len()); 
}

static WHITE_SUITS: [Suit; 2] = [Suit::DIAMONDS, Suit::HEARTS];
static BLACK_SUITS: [Suit; 2] = [Suit::CLUBS, Suit::SPADES];
static RANKS: [Rank; 13] = [Rank::ACE, Rank::TWO, Rank::THREE, Rank::FOUR, Rank::FIVE, Rank::SIX, Rank::SEVEN, Rank::EIGHT, Rank::NINE, Rank::TEN, Rank::JACK, Rank::QUEEN, Rank::KING]; 

fn build_poker_deck(include_jokers: bool) -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(54);

    for s in WHITE_SUITS.iter() {
        for r in RANKS.iter() {
            deck.push(Card {
                suit: *s,
                rank: *r,
                color: 0
            });
        }
    }

    for s in BLACK_SUITS.iter() {
        for r in RANKS.iter() {
            deck.push(Card {
                suit: *s,
                rank: *r,
                color: 1
            });
        }
    }

    if include_jokers {
        deck.push(Card {
            suit: Suit::JOKER,
            rank: Rank::JOKER,
            color: 0,
        });
        deck.push(Card {
            suit: Suit::JOKER,
            rank: Rank::JOKER,
            color: 1,
        });
    }

    deck
}

#[derive(Debug, Copy, Clone)]
enum Suit {
    DIAMONDS = 0,
    CLUBS    = 1,
    HEARTS   = 2,
    SPADES   = 3,
    JOKER    = 4,
}

#[derive(Debug, Copy, Clone)]
enum Rank {
    ACE   = 0,
    TWO   = 1,
    THREE = 2,
    FOUR  = 3,
    FIVE  = 4,
    SIX   = 5,
    SEVEN = 6,
    EIGHT = 7,
    NINE  = 8,
    TEN   = 10,
    JACK  = 11,
    QUEEN = 12,
    KING  = 13,
    JOKER = 14,
}

struct Card {
    suit: Suit,
    rank: Rank,
    color: u8,
}

fn render_card(card: &Card) {
    let rendered_suit = if card.color == 0 {
        match card.suit {
            Suit::DIAMONDS => '♢',
            Suit::CLUBS =>    '♧',
            Suit::HEARTS =>   '♡',
            Suit::SPADES =>   '♤',
            Suit::JOKER  =>   '☆',
        }
    } else {
        match card.suit {
            Suit::DIAMONDS => '♦',
            Suit::CLUBS =>    '♣',
            Suit::HEARTS =>   '♥',
            Suit::SPADES =>   '♠',
            Suit::JOKER  =>   '★',
        }
    };

    let rendered_rank = match card.rank {
        Rank::ACE => "A",
        Rank::TWO => "2",
        Rank::THREE => "3",
        Rank::FOUR => "4",
        Rank::FIVE => "5",
        Rank::SIX => "6",
        Rank::SEVEN => "7",
        Rank::EIGHT => "8",
        Rank::NINE => "9",
        Rank::TEN => "10",
        Rank::JACK => "J",
        Rank::QUEEN => "Q",
        Rank::KING => "K",
        Rank::JOKER => "J",
    };

    println!("┌────┐");
    if rendered_rank.len() > 1 {
        println!("│{} {}│", rendered_suit, rendered_rank);
    } else {
        println!("│{}  {}│", rendered_suit, rendered_rank);
    }
    println!("│    │");
    if rendered_rank.len() > 1 {
        println!("│{} {}│", rendered_rank, rendered_suit);
    } else {
        println!("│{}  {}│", rendered_rank, rendered_suit);
    }
    println!("└────┘");
}

