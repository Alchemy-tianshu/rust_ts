fn main() {
   // let number = 3;

   // if number < 5 {
   //     println!("condition was true");
   // }else {
   //     println!("condition was false");
   // }
    
    //  if后的表达式必须要产生bool值，否则会报错
    //  以下代码会报错
    //if number {
    //    println!("number was three");
    //}
    //else if多重循环
    //let number = 6;

    //if number % 4 == 0 {
    //    println!("number is divisible by 4");
    //}else if number % 3 == 0 {
    //    println!("number is divisible by 3");
    //}else if number % 2 == 0 {
    //    println!("number is divisible by 2");
    //}else {
    //    println!("number is not disvisible by 4,3,or 2");
    //}

    let condition = true;

    let number = if condition {
        5//if else中的值的类型要一样，不一样会报错
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

