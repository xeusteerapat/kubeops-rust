fn main() {
    let x = 5;
    println!("Value of x is: {}", x); // {} wiil interpolate value of x

    // x = 6; // Error: cannot assign twice to immutable variable `x`
    let mut y = 3; // in case you want to mutate variable
    println!("Value of y is: {}", y);
    y = 6;
    println!("Value of y is: {}", y);

    // Contants ALWAYS IMMUTABLE!
    const MY_AGE: u32 = 35;
    println!("My age is: {} years old", MY_AGE);

    // Shadowing variables
    let a = 2;
    let a = a + 1; // 2 + 1 = 3
    println!("Value of a is: {}", a);

    let a = a * 2; // 3 * 2 = 6
    println!("Value of a is: {}", a);

    // Shadowing and change date type
    let my_name = "Teerapat";
    println!("My name: {}", my_name); // Teerapat
    let my_name = my_name.len();
    println!("My name: {}", my_name); // 8 -> length of Teerapat

    // But if you set mut variable, you'll get warning
    let mut my_nickname = "Xeus";
    println!("My nickname: {}", my_nickname); // Still can get Xeus, but you should not do this
    let my_nickname = my_nickname.len();
    println!("My nickname: {}", my_nickname); // 4 -> length of Xeus
}
