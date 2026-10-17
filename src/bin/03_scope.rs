fn main() {
    let cristiano: i32 = 7;
    {
        let messi: i32 = 10;
        println!("cristiano wears {} and messi wears {}", cristiano, messi);
        // this should run cuz both variables cristiano and messi are within the scope
    }
    // println!("cristiano wears {} and messi wears {}", cristiano, messi);
    // will throw an error - the binding `messi` is available in a different scope in the same function
}