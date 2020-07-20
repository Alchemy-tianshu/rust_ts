//fn main() {
//    loop {
//        println!("again!");
//    }
//}

//loop循环
//fn main() {
//    let mut counter = 0;
//
//    let result = loop {
//        counter += 1;
//
//        if counter == 10 {
//            break counter * 2;
//        }
//    };
//
//    println!("The result is {}", result);
//}


//while循环
//
//fn main() {
//    let mut number = 3;
//
//    while number != 0 {
//        println!("{}",number);
//
//        number = number - 1;
//    }
//    println!("LIFTOFF!!");
//}
//

//fn main() {
//    let a = [10, 20, 30, 40, 60];
//
//    let mut index = 0;
//
//    while index < 5 {
//        println!("the value is:{}", a[index]);
//        
//        index = index + 1;
//    }
//}
//
//
//for循环
//fn main() {
//    let a = [10, 20, 30, 40, 50];
//
//    for element in a.iter() {
//        println!("the value is:{}",element);
//    }
//}
//
//
fn main() {
    for number in (1..4).rev() {
        println!("{}!",number);
    }
    println!("LIFTOFF!!!");
}

