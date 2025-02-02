use std::fs;
use std::io::Error;

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    println!("String as bytes:  {:#?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        //going through the references of each item in the list of bytes.
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[0..];
}
fn extract_errors(log: &str) -> Vec<&str> {
    let messages: Vec<&str> = log.split('\n').collect();
    let mut errors = vec![];
    println!("Splitting messages {:#?}", messages);

    for line in messages {
        if line.starts_with("ERROR") {
            errors.push(line);
        }
    }
    errors
}
fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.text", error_logs.join("\n"))?;
    println!("{}", text.len());

    Ok(())

    // match fs::read_to_string("logs.txt") {
    //     Ok(content) => {
    //         println!("Text that was read {:#?}", content.len());
    //         let mut error_messages = extract_errors(content.as_str());

    //         //Writing Errors to error_logs.txt
    //         match fs::write("../error_logs.txt", error_messages.join("\n")) {
    //             Ok(..) => println!("Errors written Successfuly"),
    //             Err(err) => println!("There was a problem writing errors to file: {:#?}", err),
    //         }
    //     }

    //     Err(error) => {
    //         eprintln!("Reading file failed because {}", error);
    //     }
    // }
}
