use std::io::Read;
use std::collections::HashSet;

fn main() {
    let word: Vec<char> = "Secretmessage".chars().collect();
    let mut found: Vec<char> = "*".repeat(word.len()).chars().collect();

    let mut guessed = HashSet::new();

    while let Some(c) = std::io::stdin().bytes().next().and_then(|result| result.ok()).map(|byte| byte as char) {
        if c == '\n' {
            continue;
        } else if guessed.contains(&c) {
            println!("Already guessed {}", c);
        } else {
            guessed.insert(c);
            for i in 0..word.len() {
                if word[i] == c {
                    found[i] = word[i];
                }
            }
        }

        println!("{:?}", found);

        if word == found {
            break;
        }
    }

    println!("Done! Gussed {:?} letters.", guessed.len())
}
