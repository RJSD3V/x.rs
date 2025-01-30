use std::fs;

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        //going through the references of each item in the list of bytes.
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[0..];
}
fn extract_errors(log: String) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();
    let messages: Vec<&str> = log.split('\n').collect();
    println!("Split messages {:#?}", messages);

    for line in messages {
        if line.starts_with("ERROR") {
            errors.push(line.to_string());
        }
    }
    errors
}
fn main() {
    match fs::read_to_string("/Users/raajassode/dev/x.rs/logs/logs.txt") {
        Ok(content) => {
            println!("Text that was read {:#?}", content.len());
            let error_messages = extract_errors(content);
            println!("Error Messages Retrieved {:#?}", error_messages);
        }

        Err(error) => {
            eprintln!("Reading file failed because {}", error);
        }
    }
    let sentence = String::from("Hello world, How are you?");
    println!("First word of sentence:  {}", first_word(&sentence));
}
