
/*
    1、if else
    2、for 循环
    3、continue
    4、break
    5、while 循环
    6、loop 循环
*/ 


fn main() {
    // 1.1 if else
    let condition = true;
    // if else判断，成立则执行 A 代码块，否则执行 B 代码块，
    // 最后将代码块中的数字绑定给 number 变量
    let number = if condition {
        5
    }else{
        6
    };
    println!("The value of number is: {}", number);

    // 注意: 语句块的表达式，返回值的类型必须要相同，否则会报错
    // expected integer, found `&str`
    // let obj = if condition{
    //     5 // 整型
    // }else{
    //     "tank"  // 字符串类型
    // };

    // 1.2 else if 处理多重条件  （后期使用match模式匹配替代）
    let n = 6;
    if n % 4 == 0{  // 判断 n ➗ 4 是否余数为 0;
        println!("nunber is divisible by 4");
    }else if n % 3 == 0{ // 判断 n ➗ 3 是否余数为 0;
        println!("nunber is divisible by 3");
    }else if n % 2 == 0{ // 判断 n ➗ 2 是否余数为 0;
        println!("nunber is divisible by 2");
    }else{
        println!("nunber is not divisible by 4, 3, or 2");
    }

    // 2.1 for循环
    for i in 1..=5{  // for 元素 in 集合 1..5 --> [1, 2, 3, 4, 5]
        println!("{}", i);  // (转移所有权)
    }

    // 2.2 for循环一般遍历引用类型的集合，若不使用引用的话，集合的所有权会被转移，导致集合无法再次使用;
    // for item in &container{}  // &container  （不可变借用）
    // 注意: 对于有 Copy特征的类型，直接对其进行拷贝，不会转移所有权;

    // 2.3 在循环中修改元素，使用mut关键字 
    // for item in &mut collection{}  （可变借用）

    /*
    2.4 总结:
                 使用方法                    等价使用方式                            所有权
        for item in collection   for item in IntoIterator::into_iter(collection)   转移
        for item in &collection  for item in collection.iter()                 不可变借用
        for item in &mut collection    for item in collection.iter_mut()       可变借用
    */

    // 2.5 获取元素的索引
    let a = [1, 2, 3, 4, 5];
    // .iter() 方法把 a 数组变成一个迭代器
    // .enumerate() 变成 (索引, 值) 的枚举
    for (i, v) in a.iter().enumerate(){
        println!("第{}个元素是{}", i + 1, v);
    }

    // 单纯循环
    for _ in 0..10{
        // 循环10次
    }

    // 2.6 两次循环方式的优劣对比
    /*
        let collection = [1,2,3,4,5];
        - 第一种
        for i in 0..collection.len(){
            let item = collection[i];
        }
        性能: 使用 collection[i] 索引访问，会因为边界检查导致运行性能消耗;
        安全: 非连续性的索引访问，可能会访问两次，导致脏数据产生;

        - 第二种
        for i in collection{
            // ...
        }
        性能: 不会出发边界检查;
        安全: 迭代依次访问，因此不存在出现脏数据的风险;
    */

    // 3. continue 跳出本层循环

    // 4. break 结束整个循环

    // 5. while循环
    let mut n = 0;
    while n <= 5{
        println!("{}", n);
        n = n + 1;
    }
    println!("出来了!");

    // 组合实现  loop + if + break
    let mut n2 = 0;
    loop {
        if n2 > 5 {
            break
        }
        println!("{}", n2);
        n2 += 1;
    }
    println!("出来了2!");

    // while VS for
    // for循环不会通过索引去访问元素，因此更加安全与简洁，同时避免 运行时的边界检查，性能更高!

    // 6. loop 循环
    // loop {  // 死循环
    //     println!("again!!!");
    // }
    
    // break 可以单独使用，也可以带一个返回值，类似return;
    // loop 是一个表达式，因此也可以返回一个值;
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

}
