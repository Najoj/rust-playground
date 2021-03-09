use std::io::Read;

fn main() {
    let word: Vec<char> = "Secretmessage".chars().collect();
    let mut found: Vec<char> = "*".repeat(word.len()).chars().collect();
    let mut guesses: u64 = 0;

    while let Some(c) = std::io::stdin().bytes().next().and_then(|result| result.ok()).map(|byte| byte as char) {
        if 'c' == '\n' {
            guesses -= 1;
        } else {
            guesses += 1;
            for i in 0..word.len() {
                if word[i] == c {
                    found[i] = word[i];
                }
            }

            println!("{:?}", found);
            if word == found {
                break;
            }
        }

    }
    println!("Gussed letters {:?}", guesses)
}
