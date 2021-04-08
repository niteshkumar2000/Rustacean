use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    v.pop();

    for val in &mut v{
        *val += 1;
        println!("{}", val);
    }

    let v = vec![1, 2, 3, 4, 5];
    
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut welcome = String::from("hello,");
    welcome.push_str(" world");

    println!("{}", welcome);

    let slice = &welcome[0..5];
    println!("{}", slice);

    for chars in slice.chars() {
        println!("{}", chars);
    }

    let mut student_profile = HashMap::new();
    student_profile.insert(String::from("Niti"), 20);
    student_profile.insert(String::from("Jei"), 10);
    // Update
    student_profile.insert(String::from("Jei"), 30);
    // check and insert
    student_profile.entry(String::from("Ash")).or_insert(50);

    for (name, grade) in &student_profile {
        println!("{} -> {}", name, grade);
    }

    let niti_grade = student_profile.get(&String::from("Niti"));
    println!("{:?}", niti_grade);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();


    for (name, grade) in &scores {
        println!("{} -> {}", name, grade);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
