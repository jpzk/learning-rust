use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);

    let v2 = vec![1, 2, 3];

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v3 = vec![1, 2, 3, 4, 5];
    v3.push(6);

    for i in &mut v3 {
        *i += 50;
    }
    for i in &v3 {
        println!("{}", i)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match row.get(0) { 
        Some(i) => match i {
            SpreadsheetCell::Int(i) => println!("{}", i),
            _ => ()
        }
        None => ()
    }

    let literal = "initial contents";
    let mut s = literal.to_string();
    let  s2 = "directly on it".to_string();
    s.push_str(&s2);
    println!("{}", s);

    let str1 = String::from("Hello, ");
    let str2 = String::from("world!");
    let str3 = str1 + &str2;
    println!("{}", str3);
    println!("{}", str2);

    let s = format!("{}-{}", str2, str3);
    println!("{}",s);

    let mut scores = HashMap::new();
    scores.insert("Blue",10);
    scores.insert("Red", 20);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let score = scores2.get("Blue");
    match score { 
        Some(s) => println!("{}", s),
        _ => ()
    }
    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }
    scores.entry("Blue").or_insert(0);
    scores.entry("Yellow").or_insert(50);
    scores.entry("Purple").or_insert(50);

    println!("{:?}", scores);

    

}
