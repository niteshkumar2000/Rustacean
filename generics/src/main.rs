use std::fmt;

fn get_largest_element<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for el in list {
        if el > largest {
            largest = el;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> fmt::Display for Point<T> 
where T: fmt::Display, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Point (x: {}, y: {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

fn longest<'a>(x: &'a str,  y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_static_lifetime() -> &'static str {
    let s = "Hello mama!";
    s
}

fn main() {
    let list = vec![10, 23, 24, 56, 78, 98];
    println!("{}", get_largest_element(&list));

    let p1 = Point {x: 10, y: 20};
    let p2 = Point {x: 1.7, y: 2.8};

    println!("{:#?}, {}", p1, p2);

    let p3 = MixedPoint {x: 10, y: 1.5};
    let p4 = MixedPoint {x: 1.5, y: 10};

    println!("{:#?}, {:?}", p3, p4);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    /*let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // str2 does not long live :)
    }
    println!("The longest string is {}", result); */

    println!("{}", get_static_lifetime());

    
}


