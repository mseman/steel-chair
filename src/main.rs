use rand::rngs::Xoshiro128PlusPlus;
use steel_chair::{clash::get_winner, wrestler::Wrestler};

fn main() {
    println!("Tonight's main event will be The Rock versus Stone Cold Steve Austin!");

    let the_rock = Wrestler::try_new(99).unwrap();
    let stone_cold = Wrestler::try_new(99).unwrap();
    let mut rng: Xoshiro128PlusPlus = rand::make_rng();

    let winner = get_winner(&the_rock, &stone_cold, &mut rng);

    if winner.eq(&the_rock) {
        println!("The Rock wins!");
    } else {
        println!("Stone Cold Steve Austin wins!");
    }
}
