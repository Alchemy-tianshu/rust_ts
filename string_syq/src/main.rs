//fn main() {
//    let s1 = String::from("Hello");
//
//    let len = calculate_length(&s1);
//
//    println!("The length of'{}' is {}", s1, len);
//
//}
//
//fn calculate_length(s: &String) -> usize {
//    s.len()
//}
//
//
//
//fn main() {
    //let mut  s = String::from("Hello");
    //这样会报错
    //change(&s);
    
    //change(&mut s);
    
    //let r1 = &mut s;
    //let r2 = &mut s;
//}

fn main() {
    let mut s = String::from("Hello");

    {
        let r1 = &mut s;
    }

    let r2 = &mut s;
}

//fn change(some_string: &mut String) {
    //some_string.push_str(", world");
//}


