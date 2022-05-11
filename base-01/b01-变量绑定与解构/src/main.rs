struct Struct {
    e: i32  // 32byte int type
}

fn main() {
    // 2022-05-10
    // 1.变量绑定，因为rust中有所有权的概念，当该变量值绑定给对应的变量，只允许当前绑定的变量使用
    let x = 5; 
    println!("value: {}", x);

    // //  当前x变量已经被绑定，无法再重新赋值
    // x = 6;
    // // cannot assign twice to immutable variable
    // println!("value: {}", x);  // 报错

    // 注意: rust默认的变量是不可变的

    // 2.变量可变性
    let mut y = 100;  // 通过 mut 声明y变量后面会发生改变
    println!("y value: {}", y);

    y = 250;
    println!("y new value: {}", y);

    // 3.使用下划线开头忽略未使用的变量
    // 当一个变量定义之后未使用，rust则会警告，甚至会导致变成一个bug
    // let num = 66;  // warning: `b01-变量绑定与解构` (bin "b01-变量绑定与解构") generated 2 warnings
    let _num = 66;
    let _num2 = 77;


    // 4.变量解构
    let (a, mut b):(bool,bool) = (true, false);
    // a = true 不可变，b = false 可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    // 断言
    assert_eq!(a, b);

    // 5.解构式赋值（类似于python的解压赋值）
    // rust 1.59 版本之后
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _代表匹配一个值
    // .. 接收多个值，省略变量
    [c, .., d, _] = [1,2,3,4,5];  // c = 1, _ = 5, d = 4, .. = (2,3)
    Struct {e, ..} = Struct {e: 5};
    assert_eq!([1,2,1,4,5], [a,b,c,d,e]);
    // ！！！注意: 目前赋值 += 语句还不支持解构赋值
    
    // 2022-05-11
    // 6. 常量: rust常量是不可变的，用const来定义，并且需要指定类型
    const MAX_POINTS:u32 = 100_000;
    println!("max point: {}", MAX_POINTS);

    // 7. 变量遮蔽（shadowing）
    // rust允许声明相同变量名，最后面声明的会遮蔽前面声明相同的变量名
    // 变量遮蔽的作用就是，可以重复使用变量名，无须去记更多变量的名字。
    let shadowing_num = 5;
    // 在main函数的作用域中对之前的 shaowing_num 变量进行遮蔽
    let shadowing_num = shadowing_num + 245;
    
    {
        // 在当前花括号作用域内，对之前的 shaowing_num 进行遮蔽
        let shadowing_num = shadowing_num * 2;
        println!("inner shadowing value is: {}", shadowing_num);  // 在花括号内的作用域有效
    }

    println!("outer shadowing value is: {}", shadowing_num);  // 在main函数内的作用域有效

    let spaces = "    "; // 字符串
    let _spaces = spaces.len();  // usize数值类型

    // 若不用let重新绑定相同的变量名则会报错
    // let mut spaces = "     ";
    // spaces = spaces.len();  //  expected `&str`, found `usize`

    // !!! 注意: rust的数据类型非常严格，不允许将usize类型赋值给字符串类型，usize是一种cpu相关的整数类型。


    // 练习: 
    // 1.绑定和可变性

    // 1.1未初始化，未使用也会报错，可以通过 _ 设置为忽略变量
    let x:i32 = 100; 
    // let x2:i32;  
    let _y:i32;  
    println!("x1 is equal to {}", x);

    // 1.2使用 mut 标记为可变
    let mut x = 1;
    x += 1;
    println!("x = {}", x);

    // 2.变量作用域

    // 2.1作用域是一个变量在程序中能够保持合法的范围
    let x1:i32 = 10;
    {
        let y1:i32 = 20;
        println!("x value is {}, y value is {}", x1, y1);
    }

    // y1只能在花括号内使用，在外层无法使用
    // println!("x value is {}, y value is {}", x1, y1);

    let y1:i32 = 30;
    println!("x value is {}, y value is {}", x1, y1);

    // 2.2 修复错误
    let x2 = "hello";
    println!("{}, world", x2);  

    // 3.变量遮蔽
    // 3.1 只允许修改 assert_eq 来让 println! 工作
    // let x3: i32 = 5;
    // {
    //     let x3 = 12;
    //     assert_eq!(x3, 5);  // 断言失败，因为x3的 12 不等于 5
    // }

    // assert_eq!(x3, 12);  // 断言失败，因为x3的 5 不等于 12
    // let x3 = 42;
    // println!("{}", x3);  // 输出42

    // 3.2 删除一段代码以通过编译
    // let mut x4:i32 = 1;
    // x4 = 11;
    // // 遮蔽再次绑定
    // let x4 = x4;  
    // // x4 += 6; // mut 才是可变的

    // let y4 = 4;
    // let y = "I can also be bound to text!";


    // 4.未使用的变量
    // let x5 = 1; // warning: unused variable: `x5`

    // 5.变量解构
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);


    // 6.解构式赋值
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];

    // 填空，让代码工作
    assert_eq!([x,y], [3, 2]);
    
}

// fn define_x2(){
//     let x2 = "hello";
// }
