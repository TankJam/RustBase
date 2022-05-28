use num::complex::Complex; // 导入num库

// 基本数据类型
/*
    数值类型:
        有符号: i8, i16, i32, i64, isize

        无符号: u8, u16, u32, u64, usize

        浮点数: f32, f64

        理数、复数

    字符串:
        &str

    布尔类型:
        true, false

    字符类型:
        Unicode, 单个字符为4个字节

    单元类型:
        ()

    类型推导 与 标注
        - 在大多数情况下，rust编译器都能根据变量的使用自动推导出变量的类型
        - 但是在某种情况下无法推导，所以需要手动去做一个类型的标注
*/

// 数值类型
fn number_type(){
// 类型推导 与 标注
    // 编辑器不知道这个到底是字符串、整形还是浮点型
    // let guess = "9527".parse().expect("Not a number"); // 通过 .parse().expect 进行标注
    let guess:i32 = "9527".parse().expect("Not a number"); // 通过 .parse().expect 进行标注
    println!("value: {}", guess);

    /*
        1、整数类型:
            i: integer  有符号
                i8: -128 ~ 127
            u: unsigned 无符号
                u8: 0 ~ 255

            isize、usize 取决于CPU的类型，32与64位。

            若超出范围则类型溢出。
    */

    /*
        2、浮点类型:
            f32、f64

            浮点型的比较在精确度上会有问题
                - 避免在浮点数上测试相等性
                - 当结果在数学上可能存在未定义时，需要格外小心
    */

    let _x = 2.0;  // f64 默认是f64
    let _u:f32 = 3.0;  // f32

    // 断言 0.1 + 0.2 与 0.3相等
    // assert!(0.1 + 0.2 == 0.3); // 报错，因为小数点N位存在误差

    // 解决: 通过精确度去解决
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    println!("abc (f32)");
    // f32的精确度是一致的，所以进行比较是相同的
    println!("  0.1 + 0.2 : {:x}", (abc.0 + abc.1).to_bits());  // .to_bits() 获取精确度
    println!("        0.3 : {:x}", (abc.2).to_bits());
    println!();

    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("xyz (f64)");
    // f64的精确度是不一致的，所以进行比较会出问题
    println!("  0.1 + 0.2 : {:x}", (xyz.0 + xyz.1).to_bits());
    println!("        0.3 : {:x}", (xyz.2).to_bits());
    println!();

    // f32 与 f64 的断言
    assert!(abc.0 + abc.1 == abc.2);  // 断言通过
    // assert!(xyz.0 + xyz.1 == xyz.2);  // 断言失败

    // NaN 
    // 对于数学上未定义的结果，例如对负数取平方根 -42.1.sqrt()，会产生一个特殊的结果，rust类型为NaN（not a number）
    // NaN不能用于比较，否则会崩溃
    let x2 = (-42.0_f32).sqrt(); // 得到-42.0为f32的平方根，为NaN类型
    // assert_eq!(x2, x2);  // NaN 与 NaN 比较

    // 为了避免报错，rust提供了 si_nan() 等方法，可以用于判断一个值是否为NaN类型
    if x2.is_nan(){
        println!("未定义的数学行为!");
    }

    /*
        3、数字运算
            加、减、乘、除、求余

            注意: 只有同样类型才能进行运算

    */

    let num1 = 20;  // 默认i32类型
    let num2:i32 = 22;  //类型标注为i32
    let num3 = 24i32;  //类型后缀指定i32类型

    // 相同类型才能进行运算
    let add = num1 + num2 + num3;
    println!("{} + {} + {} = {}", num1, num2, num3, add);

    // 对于较长的数字，可以用_进行分割，提升可读性;
    let one_million:i64 = 1_000_000;
    println!("{}", one_million);


    // 定义一个f32数组，其中50.0会被自动推导为f32类型
    let float_array = [
        50.0,
        51f32,
        52.0_f32,
    ];
    // 保留两位小数格式化输出
    println!("{:.2}", float_array[0]);

    /*
        4、位运算
            &与、|或、^异或（相同位置不同则为1，相同为0）、!非、<<左移（向左移指定位数）、>>右移
    */
    // 二进制为 00000010
    let a1:i32 = 2;
    // 二进制为 00000100
    let b1:i32 = 3;
    println!("(a1 & b1) value is {}", a1 & b1);
    println!("(a1 | b1) value is {}", a1 | b1);
    println!("(a1 ^ b1) value is {}", a1 ^ b1);  // 比较两个变量不同为1，相同为0
    println!("(a1 ! b1) value is {}", !b1); // 位取反 -4
    println!("(a1 << b1) value is {}", a1 << b1); // 二进制为 00000010000  16
    println!("(a1 >> b1) value is {}", a1 >> b1); // 二进制为 0000000.1     0
    // 注意: 除了 ! 运算符以外，其他都可以加上 = 进行赋值
    let mut a1 = a1;  // 定义可变量a2
    a1 <<= b1;
    println!("(a1 << b1) value is {}", a1);


    /*
        5、序列（Range）
            Rust提供了一个非常简洁的方式，用来生成连续的数值，例如 1..5，生成从1-4的连续数字，不包含5，顾头不顾尾;

            0-N
            a-z
    */
    // 1..=5 包含5
    for i in 1..=5 {  
        println!("{}", i);
    }

    // 1..5 不包含5  顾头不顾尾
    for i2 in 1..5{
        println!("{}", i2)
    }

    // 注意: 序列只允许用于数字或字符类型，原因是可以连续，同时编译器在编译时可以检查该序列是否为空，
    // 字符和数字值是rust中仅有可以判断是否为空的类型
    for i3 in 'a'..='z' {
        println!("{}", i3);
    }

    /*
        6、有理数和复数
            - 有理数和复数。
            - 任意大小的整数和任意精度的浮点数。
            - 固定精度的十进制小数，常用于货币相关的场景。
        
        - 通过第三方的库来实现: num
            1.在Cargo.toml 中的 [dependencies] 下添加一行 num = "0.4.0" ;
            2.在src/main.rs 文件的开头 通过 use num::complex::Complex; 方式导入;
            3.编写代码，并编译执行;
    */
    let d = Complex {re: 2.1, im: -1.2};
    let e = Complex::new(11.1, 22.2);
    let result = d + e;  // d.re + e[0] , d.im + e[1]
    println!("{} = {}", result.re, result.im); // 13.2 = 21

    /*
    数值类型总结:
        rust的数值类型与其他语言较为相似，但是实际上，除了语法不同之外，还存在一些差异:
            1.Rust 拥有相当多的数值类型，因此需要熟悉这些类型所占用的字节数，才能知道不同类型的大小范围、是否为负数;
            2.类型转换必须是显式的，rust 永远也不会偷偷将 16bit 整数转为 32bit 的整数;
            3.Rust的数值上可以使用方法, 例如可以用以下的方法来将 13.14 取整， 13.14_f32.round(), 但前提要通过
            _f32指定类型，这样编译器才能知道13.14的具体类型;
    */
}

fn main(){
    // // 数值类型
    // number_type()

    
}
