pub mod elision_rule_three;
pub mod basic_generic_lifetime;
pub mod lifetime_and_generic_example;

fn main() {
    basic_generic_lifetime::run();
    elision_rule_three::run();
    lifetime_and_generic_example::run();
}
