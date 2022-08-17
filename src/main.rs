use rand::Rng;
use std::io;

fn main() {
    //let options = ["Schere", "Stein", "Papier"];
    let mut user_options = String::new();

    println!("Select Operation: (1) Schere (2) Stein (3) Papier: ");
    io::stdin()
        .read_line(&mut user_options)
        .expect("failed user input"); //&mut (Mutation) kann nicht geändert werden | io Modul (Input / Output)

    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1..3);
    println!("Computer: {}", random);

    let user = match user_options.trim().parse() {
        Ok(i) => i,
        Err(_e) => -1,
    };

    println!("User: {}", user);

    if random == user {
        println!("Sie haben ein unentschieden gespielt!")
    } else if user == 1 && random == 2 {
        println!("Sie haben leider verloren!")
    } else if user == 2 && random == 3 {
        println!("Sie haben leider verloren!")
    } else if user == 3 && random == 1 {
        println!("Sie haben leider verloren!")
    } else {
        println!("Gratuliere, sie haben gewonnen!")
    }
}
