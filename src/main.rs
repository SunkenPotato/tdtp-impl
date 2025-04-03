use rand::Rng;

fn main() {
    let rng: usize = rand::rng().random_range(1..10);

    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf);

    if buf.trim().parse::<usize>().unwrap() == rng {
        println!("Guessed correctly")
    } else {
        println!("Incorrect, the number was {rng}")
    }



}

