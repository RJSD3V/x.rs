fn print_elements(elements: &[String]) {
    // for (i, item) in elements.iter().enumerate() {
    //     println!("For {:#?} at {:#?}", i, item);
    // }

    elements.iter().for_each(|el| println!("{}", el));
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1))
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("Blue"),
    ];

    // print_elements(&colors);
    // shorten_strings(&mut colors);
    // println!("{:#?}", colors);
    // let uppercased = to_uppercase(&colors);
    // println!("{:#?}", uppercased);

    // let mut dest_vec = vec![];
    // move_elements(colors, &mut dest_vec);

    // println!("Destination: {:#?}", dest_vec);
    println!("String Vector Exploded {:#?}", explode(&colors));

    let found_color = find_color_or(&colors, "re", "Orange");
    println!("Color Found -> {:#?}", found_color);
}
