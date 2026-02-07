fn borrow_vec(vector: &Vec<i32>) {
    println!("{:?}", vector);
}

fn own_integer(mut x: i32) -> i32{
    x = x + 1;
    x
}

fn own_string(s: &String) {
    println!("{}", s);
}

fn own_vec(mut vec: Vec<i32>) -> Vec<i32>{
   vec.push(6);
   vec.to_vec()
}

// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function 
// or another part of your program without actually transferring ownership of the variable. 
// When you borrow a variable, you're essentially saying 
// "I want to use this variable for a little while, but I promise I won't modify it."
fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, world!");

    // this compiles no problem!
    own_integer(my_int);
    println!("{}", my_int);

    own_string(&my_string); // take ownership of my_string
    // this is using my_string which has also moved and is invalid
    println!("{:?}", my_string); // this will not compile!

    borrow_vec(&my_vec);
    // but this is using my_vec which was borrowed (moved) and yet is now invalid
    println!("{:?}", my_vec); // this will not compile!

    let new_vec = own_vec(my_vec);
    println!("my new vec {:?}", new_vec)
}

// Borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient. 
// By lending ownership of a variable instead of transferring it, Rust ensures that only 
// one part of your program can modify the variable at a time, which helps prevent 
// bugs and makes it easier to reason about your code.