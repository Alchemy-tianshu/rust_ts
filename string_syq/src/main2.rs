//fn main() {
//    let mut s = String::from("Hello");
//
//    s.push_str(", world!");   //push_str函数向String空间的尾部添加了一段字面量
//
//    println!("{}", s)   //这里会输出完整的hello world!
//}
//
//
fn main() {
    let s1 = String::from("Hello");
    
    //s1被移动到了s2，s1被废弃了，调用s1会报错
    //let s2 = s1;
    
    //会复制栈内存和对内存中对所有数据
    let s2 = s1.clone();
    
    //报错
    //println!("{}, world", s1);

    //正常
    //s1被移动到了s2
    println!("{}, world", s2);


    //克隆
    println!("s1 = {}, s2 = {}", s1, s2);
}
