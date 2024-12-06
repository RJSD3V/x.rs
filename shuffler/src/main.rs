
use rand::{thread_rng, seq::SliceRandom};
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {

            //list of suits - Clubs, Hearts, Spades 
    // An Array has a fixed size and is only used by declaring a []
    // A Vector is an array you can change. - declared using vec![]
    let suits = ["Hearts","Spades","Clubs", "Diamonds"];
    // List of 'values' - ace, 'twos', etc.
    let values = ["Ace","Two","Three","Four","Five","Six","Seven","Eight","Nine","Ten","Jack","Queen","King"];

    let mut cards = vec![];
    for suit in suits{
        for value in values {
            let card = format!("{} of {}", value, suit);

            cards.push(card);
        }

  
    }

        Deck {cards}

    

    }

    fn shuffle(&mut self){

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);

    }


    fn deal(&mut self, num_cards: usize) -> Vec<String>{

        self.cards.split_off(
            self.cards.len() - num_cards
        )

    }
}

fn main() {


    let mut deck = Deck::new();


    // deck.shuffle();

    let cards = deck.deal(3);

    println!("HEre's your cards dealt {:#?}", cards);
    println!("Here's your deck {:#?}", deck);
}