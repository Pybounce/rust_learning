fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    let x_plus_one = plus_one(x);
    println!("The value of x + 1 is: {x_plus_one}");
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn what_type() {    //making the compiler throw an error can tell you the type - the cheese has already begun.
    let mut strVar = "s";
    strVar = "asfasf";
    strVar.yeet();
}