use rand::Rng;
use std::{thread, time};
use std::io::{stdin, stdout, Read, Write};

fn pause() {
            let mut stdout = stdout();
            stdout.write(b"Press Enter to continue.....").unwrap();
            stdout.flush().unwrap();
            stdin().read(&mut [0]).unwrap();
    }

fn main() {
    println!("Flip a coin!");
    
    // Define the variable for our while loop
    let mut n = 5;
    
    // Pause until a key is pressed, defined in out pause function
    pause();

    while n > 0 {
    
    // Sleep for 1 second
    let one_sec = time::Duration::from_millis(1000);
    let now = time::Instant::now();
    thread::sleep(one_sec);
    assert!(now.elapsed() >= one_sec);
    
    // Print the num as it changes
    println!("{}", n);
    
    // Define how to value changes
    n-=1
    }
    
    // Random a number between 0 and 1, 0 is our Heads, and 1 is our Tails
    let secret_flip = rand::thread_rng().gen_range(0..=1);
    if {secret_flip} == 0 {
        println!("Heads!")
    } 
    else {
        println!("Tails!")
    }

}
