fn main() {
    // integer
    let x: i32 = -16;
    println!("x is: {}", x);
    let mut y: i8 = 120;
    y = y + 1; // dont add 55 
    println!("y is: {}", y);
    
    // float
    let x = 2.1; // f64
    let y: f32 = 3.1; // f32
    println!("x: {} and y: {}", x, y);

    // tuples
    let tup: (i32, bool, char) = (1, true, 's');
    println!("tup: ({},{},{})", tup.0, tup.1, tup.2);

    // arrays
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("Month: {}", months[10]);
 
}
