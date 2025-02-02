use std::time::{Duration, Instant};
use std::thread::sleep;

//We have several types of formatted print in Rust. 
// - format!: write formatted text to String
// - print! : same as format! but the text is printed to the console (io::stdout)
// - println! : same as print! but a newline is appended
// - eprint! : same as print! but the text is prented to the standard error (io::stderr)
// - eprintln! : same as println! but the text has a new line appended to it. 
fn main(){
    sleep(Duration::new(2,0));
    //IN general , {} will be replaced with any arguments. These will be stringified.
    let now = Instant::now();
    let now_before = &now.elapsed().as_secs();
    println!("{} is the current time.", now_before.to_string() );
    //Introduce a sleep to make this a simulation of a process.
    sleep(Duration::new(2,0));
    //Positional arguments can be used. 
    let elapsed = &now.elapsed().as_secs();
    let elapsed_str = elapsed.to_string();
    println!("{0} second skip to simulate a code execution", elapsed_str);
 let after = Instant::now();
 println!("{:?} seconds",  after.duration_since(now));
}
