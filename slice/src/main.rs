fn main() {
    //let s = String::from("hello world");

   // let a = &s [0..5];
    
    //let c = &s [..5];
    //println!("{}!", c);
    
    //let d = &s [..];
   // println!("{}!!!", d);
    //let b = &s [6..11];

    //println!("{}.{}",a, b);
    
    let a = [1, 2, 3, 4, 5];

    let slice = &a [1..3];

    println!("{}", slice[1]);
}
