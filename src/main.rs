use rand::Rng;
use std::time::Instant;
use num::BigUint;
use num::pow::pow;

// A function to calculate the key space of a given number of bits
fn key_space(bits: u32) -> BigUint {
    // BigUint representation of 2
    let two = BigUint::from(2u32);
    // The key space is equal to 2^bits
    pow(two, bits.try_into().unwrap())
}

// A function to brute-force the key of a given number of bits
fn brute_force(bits: u32) {
    // Calculate the key space
    let key_space = key_space(bits);
    // Try to convert the key space to a u64
    let max = match key_space.clone().try_into() {
        // If successful, assign the value to max
        Ok(x) => x,
        // If not, return from the function
        _ => return,
    };
    // Generate a random key between 0 and max
    let key = BigUint::from(rand::thread_rng().gen_range(0u64..max) as u64);
    // Record the start time of the function
    let start = Instant::now();
    // Loop through each possible key
    for i in 0u64.. {
        // If the current key is equal to the generated key
        if BigUint::from(i) == key {
            // Print the number of bits and the elapsed time
            println!("Found {}-bit key in {}ms", bits, start.elapsed().as_millis());
            // Break out of the loop
            break;
        }
    }
}


// The main function
fn main() {
    // Print the key space for 8, 16, 32, 64, 128, 256, 512, 1024, 2048, and 4096 bits
    println!("Key space for 8-bit key: {}", key_space(8));
    println!("Key space for 16-bit key: {}", key_space(16));
    println!("Key space for 32-bit key: {}", key_space(32));
    println!("Key space for 64-bit key: {}", key_space(64));
    println!("Key space for 128-bit key: {}", key_space(128));
    println!("Key space for 256-bit key: {}", key_space(256));
    println!("Key space for 512-bit key: {}", key_space(512));
    println!("Key space for 1024-bit key: {}", key_space(1024));
    println!("Key space for 2048-bit key: {}", key_space(2048));
    println!("Key space for 4096-bit key: {}", key_space(4096));
    // Brute-force the key for 8, 16, 32, 64, 128, 256, 512, 1024, 2048, and 4096 bits
    brute_force(8);
    brute_force(16);
    brute_force(32);
    brute_force(64);
    brute_force(128);
    brute_force(256);
    brute_force(512);
    brute_force(1024);
    brute_force(2048);
    brute_force(4096);
}
