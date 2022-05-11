struct Struct {
    e: i32  // 32byte int type
}

fn main() {
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

}
