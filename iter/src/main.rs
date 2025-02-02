fn print_elements(elements: &Vec<String>) {
    // for (i, item) in elements.iter().enumerate() {
    //     println!("For {:#?} at {:#?}", i, item);
    // }

    elements
        .iter()
        .map(|el| format!("This is the color {}", el))
        .for_each(|el| println!("{} in format", el));
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("Blue"),
    ];

    print_elements(&colors);
}
