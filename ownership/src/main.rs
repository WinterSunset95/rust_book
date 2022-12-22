



fn main() {
    // ---- Ownership rules ----
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // ---- Variable scope ----
    {
        let s = "Hello"; // s is valid from this point forward
        println!("{}", s); 
                         // do stuff with s
    } // this scope is now over, and s is no longer valid

    // Shallow copy
    {
        let x = 5;
        let y = x; // x is copied to y
                   // x is still valid
        println!("x = {}, y = {}", x, y);
        // both x and y are valid
    }

    {
        let s1 = String::from("Hello Rust");
        let s2 = s1; // s1 is moved to s2
                     // s1 is no longer valid
        println!("{}", s2);
    }

    // Clone method
    {
        let s1 = String::from("Hello Winter");
        let s2 = s1.clone(); // s1 is copied to s2
                             // s1 is still valid
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // ---- Ownership and functions ----
    {
        fn takes_ownership(some_string: String) {
            println!("{}", some_string);
        }

        let s = String::from("This will be moved to the takes_ownership() function");
        takes_ownership(s); // s is moved to the function
                            // s is no longer valid
    }
    {
        fn makes_copy(some_string: String) {
            println!("{}", some_string);
        }

        let s = String::from("This will be copied to the makes_copy() function");
        makes_copy(s.clone()); // s is copied to the function
                               // s is still valid
    }

    // ---- References ----
    {
        fn calculate_length(s: &String) -> usize {
            s.len() // gets the length and returns it
        }

        let s = String::from("This string is referenced");
        let len = calculate_length(&s);
        println!("{}, {}", s, len);
    }

}
