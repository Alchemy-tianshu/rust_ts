fn main() {
    let  s = String::from("Hello");  //变量s进入作用域

    takes_ownership(s);   //s的值被移动进了函数，所以它从这里开始不再有效

    let x = 5;    //变量x进入作用域

    makes_copy(x);  //变量x同样被传递进了函数，但由于i32是copy的，所以我们依然可以在这之后使用x
    
    println!("{}",x);
    //打印s会报错
   // println!("{}",s);
}   //x首先离开作用域，随后是s

//但由于s的值已经发生了移动，所以没有什么特别的事情会发生

fn takes_ownership(some_string: String) {   //some_string进入作用域
   println!("{}", some_string); 
}   //some_string在这里离开作用域，drop函数会自动调用
//some_string所占用的内存也随之被释放了
//
fn makes_copy(some_integer: i32) {   //some_integer进入作用域
    println!("{}",some_integer);
}    //some_integer在这里离开了作用域，没有什么特别的事情生



