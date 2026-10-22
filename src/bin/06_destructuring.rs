// fn main() {
//     let (mut x, y) = (1, 2); // tuple
//     x += 2;
//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
    
//     println!("success");
// }

fn main() {
    let (x, y);
    // same as writing 
    // let x;
    // let y;
    (x, ..) = (7, 10);  // tuple 
    [.., y] = [8, 37];  // array

    assert_eq!([x, y], [7, 37]);   // takes two same arguments; here it is two arrays

    println!("success!");
}