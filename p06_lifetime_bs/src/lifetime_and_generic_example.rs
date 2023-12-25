use std::fmt::Display;


pub fn run() {
    let min_str = "min_str";
    let longest_str = "longest_str";
    longest_with_an_announcement(&min_str, &longest_str, "announcement message");
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}