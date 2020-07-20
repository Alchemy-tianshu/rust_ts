fn main() {
    // 常量写法
   // const MAX_POINTS: u32 = 100_000;
    //println!("value is {}",MAX_POINTS);
    //声明不可变变量
    //let mut x = 5;
    //println!("The value of x is: {}", x);

    //x = 6;
    //println!("The value of x is: {}", x);

    //let x = 5;

    //let x = x + 1;

    //let  x = x * 2;

   // println!("The value of x is:{}",x);
   
    //隐藏的写法
    //let spaces = "    ";
    //let spaces = spaces.len();
    //println!("space is {}",spaces);
    
    //错误的写法
    //let  mut spaces = "   ";
    //spaces = spaces.len();
    //println!("space is {}",spaces);
    //正确写法
    //let guess: u32 = "42".parse().expect("Not a number!");
    //错误写法
    //let guess = "42".parse().expect("Not a number!");

    //let x = 2.0; //f64

    //let y: f32 = 3.0;   //f32

    //加法
    let sum = 5 + 10;

    //减法
    //let difference = 95.6 - 4.3;

    //乘法
    //let product = 4 * 30;

    //除法
    //let quotient = 56.7 / 32.2;

    //取余
    //let remainder = 43 % 5;

    //println!("{},{},{},{},{}", sum,difference,product,quotient,remainder);

    //布尔类型
    //let t = true;

    //let f: bool = false;

    //println!("{}",f);
    

    //let tup: (i32, f64, u8) = (500, 6.4, 1);

    //let (x, y, z) = tup;
    //let five_hundred = tup.0;

    //let six_point_four = tup.1;

    //let one = tup.2;

    //println!("{},{},{}", five_hundred, six_point_four, one);

    //println!("The value of y is: {}", y);
    
    //数组的写法
    //let a = [1,2,3,4,5];
    //println!("{}",a[0]);
    
    //数组的写法2
    //let a: [i32; 5] = [1,2,3,4,5];
    //println!("{}",a[1]);

    //数组越界
    //let a = [1,2,3,4,5];

    //let index = 10;

    //let element = a[index];

   // println!("The value of element is:{}", element);

    println!("Hello World!");

    another_function(5,6);
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
