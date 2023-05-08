use indexmap::IndexMap;
use rand::Rng;
use std::io;

fn main() {
    let mut nato_alphabet = IndexMap::new();

    nato_alphabet.insert('a', String::from("alfa"));
    nato_alphabet.insert('b', String::from("bravo"));
    nato_alphabet.insert('c', String::from("charlie"));
    nato_alphabet.insert('d', String::from("delta"));
    nato_alphabet.insert('e', String::from("echo"));
    nato_alphabet.insert('f', String::from("foxtrot"));
    nato_alphabet.insert('g', String::from("golf"));
    nato_alphabet.insert('h', String::from("hotel"));
    nato_alphabet.insert('i', String::from("india"));
    nato_alphabet.insert('j', String::from("juliett"));
    nato_alphabet.insert('k', String::from("kilo"));
    nato_alphabet.insert('l', String::from("lima"));
    nato_alphabet.insert('m', String::from("mike"));
    nato_alphabet.insert('n', String::from("november"));
    nato_alphabet.insert('o', String::from("oscar"));
    nato_alphabet.insert('p', String::from("papa"));
    nato_alphabet.insert('q', String::from("quebec"));
    nato_alphabet.insert('r', String::from("romeo"));
    nato_alphabet.insert('s', String::from("sierra"));
    nato_alphabet.insert('t', String::from("tango"));
    nato_alphabet.insert('u', String::from("uniform"));
    nato_alphabet.insert('v', String::from("victor"));
    nato_alphabet.insert('w', String::from("whisky"));
    nato_alphabet.insert('x', String::from("x-ray"));
    nato_alphabet.insert('y', String::from("yankee"));
    nato_alphabet.insert('z', String::from("zulu"));

    loop {
        let random_index = rand::thread_rng().gen_range(0..nato_alphabet.len());
        let random_kv_pair = nato_alphabet
            .get_index(random_index)
            .expect("A valid nato alphabet entry");

        println!("Enter answer for: {}", random_kv_pair.0);

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("F");

        match answer.to_lowercase().trim().eq(random_kv_pair.1) {
            true => println!("Your answer was correct!\n"),
            false => println!(
                "Your answer was incorrect. Expected answer was: {}\n",
                random_kv_pair.1
            ),
        }
    }
}
