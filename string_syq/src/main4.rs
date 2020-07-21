//fn main() {
//
//    let s1 = gives_ownership();   //gives_ownership将它的返回值移动至s1中
//
//    let s2 = String::from("hello");   //s2进入作用域
//
//    let s3 = takes_and_gives_back(s2);   //s2被移进函数
//
//    //takes_and_gives_back中，而这个函数的返回值又被移动到了变量s3上
//}//s3在这里离开作用域并被销毁。由于s2已经移动了，所以它不会在离开作用域时发生任何事情。s1最后离开作用域并被销毁
//
//fn gives_ownership() -> String {
//    //gives_ownership会将它到返回值移动至调用它到函数内
//    let some_string = String::from("Hello");   //some_string进入作用域
//
//    some_string   //some_string作为返回值移动至调用函数
//}
//
////takes_and_gives_back将取得一个String到所有权并将它作为结果返回
//fn takes_and_gives_back(a_string: String) -> String {
//    //a_string进入作用域
//    a_string     //a_string作为返回值移动至调用函数
//}
//
//
//
fn main() {
    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();   //len()会返回当前字符串到长度

    (s, length)
}
