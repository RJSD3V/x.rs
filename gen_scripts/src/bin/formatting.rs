fn main() {
    
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", 
                object = "the lazy dog",
                subject = "the quick brown fox",
                verb = "Jumps over"
    );

   //Print different Base Numbers in different formats

    println!("Base 10:                  {}", 69420);
    println!("Base 2 (Binary):          {:b}", 69420);
    println!("Base 8 (Octal):           {:o}", 69420);
    println!("Base 16 (Hex)             {:x}", 69420);

    // You can right justify the text with a specified width. This will output "   1" 

    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    println!("{number:0<5}", number=1);


    // YOu can use named arguments in the format specidier by appending a '$'

    println!("{number:0>width$}", number=1, width=3);

    
    // Rust even checks to make sure the correct number of arguments are used. 
    println!("my name is {0}, {1} {0}", "Bond", "James");

    #[derive(Debug)]
    struct Structure(i32);

    //Define structure
    let structure = Structure(32);
    println!("{:?}",structure);
}

