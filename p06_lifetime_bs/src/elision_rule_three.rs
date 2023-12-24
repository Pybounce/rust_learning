pub fn run() {
    let my_struct = MyStruct {
        some_str: "",
    };
    println!("{}", my_struct.some_method(&"hello world"));
}


struct MyStruct<'a> {
    some_str: &'a str,
}

/*
Will not compile
Because the 3rd lifetime elision rule states that, if one input is &self, then the output must be &self. But in our case, it is 
the timetime of myStr

impl<'a> MyStruct<'a> {
    fn some_method(&self, my_str: &str) -> &str {
        return my_str
    }
}
*/


//Compiles just fine
//We are explicitly stating the lifetime of the output param, so it won't be implicitly set via the elision rules.
impl<'a> MyStruct<'a> {
    fn some_method<'b>(&self, my_str: &'b str) -> &'b str {
        return my_str
    }
}