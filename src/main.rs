use rand::Rng;
use std::io;

fn main() {
    println!("Devine un nombre entre 1 et 100 !");
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Entre ta proposition :");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Échec de la lecture");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ce n'est pas un nombre, réessaie !");
                continue;
            }
        };

        if guess < secret {
            println!("Trop petit !");
        } else if guess > secret {
            println!("Trop grand !");
        } else {
            println!("Bravo, tu as trouvé !");
            break;
        }
    }
}
