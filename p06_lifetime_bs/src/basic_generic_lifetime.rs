pub fn run() {
    let largest;
    let y = 100;
    let x = 50;
    largest = get_largest(&x, &y);
    println!("{}", largest.to_string())
}


/*
Even though my code only uses this in a way where the lifetime of x and y end at the same time, the borrow checker doesn't like it.
In terms of function life times, it doesn't check outside of the function, it just cares about the function signature

fn get_largest(x: &i32, y: &i32) -> &i32 {
    if x > y {
        x
    }
    else {
        y
    }
}
*/

fn get_largest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    }
    else {
        y
    }
}
