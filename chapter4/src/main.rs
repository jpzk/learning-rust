fn main() {
    moving_ownership();
    slice_operations();
} 


fn moving_ownership() {
    let s1 = gives_ownership();         
    let s2 = String::from("hello");     
    let s3 = takes_and_gives_back(s2);  
}

fn gives_ownership() -> String {             
    let some_string = String::from("hello"); 
    some_string                              
}
fn takes_and_gives_back(a_string: String) -> String { 
    a_string  
}

//-- 4.03
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn slice_operations() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
}