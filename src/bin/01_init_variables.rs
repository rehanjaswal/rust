fn main() {
    let x: i32 = 7;
    let _y: i32; // unused and uninitialised variables, shows warning - unused variable: `y`
    // can be fixed by prefixing it with an underscore -> _y: i32;

    // if used but uninitialised, say if i hadn't initialised x as 7, the terminal would've thrown me an error
    // ERROR - `x` used here but it isn't initialized
    assert_eq!(x, 7);
    println!("success");
}