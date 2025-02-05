fn print_elements(elements: &[String]) {
    // for (i, item) in elements.iter().enumerate() {
    //     println!("For {:#?} at {:#?}", i, item);
    // }

    elements
        .iter()
        .map(|el| format!("This is the color {}", el))
        .for_each(|el| println!("{} in format", el));
}

fn shorten_strings(elements: &mut Vec<String>) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("Blue"),
    ];

    // print_elements(&colors);
    println!("{:#?}", shorten_strings(&mut colors));
}
