use std::io;
use std::io::{stdout, Write};

static ALPHABET: [(char, &str); 26]  = [
    ('a', ".-"),
    ('b', "-..."),
    ('c', "-.-."),
    ('d', "-.."),
    ('e', "."),
    ('f', "..-."),
    ('g', "--."),
    ('h', "...."),
    ('i', ".."),
    ('j', ".---"),
    ('k', "-.-"),
    ('l', ".-.."),
    ('m', "--"),
    ('n', "-."),
    ('o', "---"),
    ('p', ".--."),
    ('q', "--.-"),
    ('r', ".-."),
    ('s', "..."),
    ('t', "-"),
    ('u', "..-"),
    ('v', "...-"),
    ('w', ".--"),
    ('x', "-..-"),
    ('y', "-.--"),
    ('z', "--.."),
];

fn decode(code : &str) -> String {
    let mut result = String::new();
    let mut found = false;
    for morse_code in code.split_whitespace() {
        match morse_code {
            "" => result.push(' '),
            _ => {
                for &(letter, morse) in &ALPHABET {
                    if morse == morse_code {
                        result.push(letter);
                        found = true;
                        break;
                    }
                }
            }
        }
        if !found {
            result.push('?');
        }
    }
    return result;
}

fn encode(sentence : &str) -> String {
    let mut result = String::new();
    for c in sentence.chars() {
        match c {
            ' ' => result.push_str(" "),
            _ => {
                for &(letter, morse) in &ALPHABET {
                    if letter == c {
                        result.push_str(morse);
                        result.push(' ');
                        break;
                    }
                }
            }
        }
    }
    return result;
}

fn main() {
    loop {
        let mut choice = String::new();
        print!("1 - Encode\n2 - Decode\n3 - Exit\n> ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice : u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };
        match choice {
            1 => {
                let mut sentence = String::new();
                print!("Enter sentence to encode\n> ");
                stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut sentence)
                    .expect("Failed to read line");
                let sentence = sentence.trim();
                println!("Encoded sentence: {}", encode(sentence));
            },
            2 => {
                let mut code = String::new();
                print!("Enter morse code to decode\n> ");
                stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut code)
                    .expect("Failed to read line");
                let code = code.trim();
                println!("Decoded sentence: {}", decode(code));
            },
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}
