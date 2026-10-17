// variables are immutable by default in rust
// if i wanna mututate/change the value of a variable, i must tell rust that it's mututable using the "mut" keyword

fn main() {
    let mut x: i32 = 7;
    x += 10;  // x = 17
    // println!("{}", x); // thats how you print it
    assert_eq!(x, 17);
    println!("success");
}