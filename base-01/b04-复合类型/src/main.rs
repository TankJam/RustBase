/*
    复合类型: 符合类型由其他类型组合而成，最经典的就是结构体 struct 和 枚举enum ;

        - 字符串和切片
        - 元组
        - 结构体
        - 枚举
        - 数组
*/

// 复合类型初体验

#![allow(unused_variables)]  // 引入属性，避免声明变量后未使用导致warning警告
type ExampleFile = String; 

fn open(f: &mut ExampleFile) -> bool{
    true
}

fn close(f: &mut ExampleFile) -> bool{
    true
}

#[allow(dead_code)]
fn read(f: &mut ExampleFile, save_to: &mut Vec<u8>) -> !{  // 返回一个 ! 类型，表明该函数是一个 发散函数，不会返回任何值
    unimplemented!()  // 告诉编译器，该函数尚未实现
}

fn example(){
    let mut f1 = ExampleFile::from("f1.txt"); // 定义一个文件File类型，并且初始化为 f1.txt 文件
    open(&mut f1);
    // read(&mut f1, &mut vec![]);
    close(&mut f1);

    // 以上代码体现出只是用基本类型的局限性: 无法从更高级的抽象层次去简化代码，比如: f1.open()、 f1.close();
    // 因此我们需要使用更高级的数据类型来解决: struct 
}


/*
    1、string 和 &str （字符串和切片）
    注意: Rust中的字符串是非常重要的 ”重点“
*/
fn string_main(){
    // // String 和 &str
    string_and_str();

    // // 切片Slice 
    slice();

    // // string 与 &str 转换
    string_change_str();

    // // 字符换索引
    str_index();

    // // 字符串操作
    string_function();

    // // 字符串转义
    string_transfer();

    // // 操作UTF-8字符串
    doing_utf8_str();

    // // 字符串剖析
    str_question();

    // // 字符串作业
    str_work();
}

// 1.1 字符串
fn string_and_str(){
    let my_name = "tank jam";  // 定义一个不可变的静态字符串类型
    // greet 需要接收一个 String
    // greet(my_name) // 报错: expected struct `String`, found `&str`
}

// fn greet(name: String){ // 接收动态字符串
fn _greet(name: String){ // 接收动态字符串
    println!("hello, {}", name);
}


// 1.2 切片Slice
// 切片是对String类型中的一部分进行引用
fn slice(){
    let s = String::from("hello world!");
    let hello = &s[0..5];  // 引用s动态字符串中的索引为 0-4的值， 通过[0..5] 来绑定引用的值给hello变量
    let world = &s[6..];  // 引用从下表为6的索引，到最后一个索引的值
    println!("hello: {}  - world: {}", hello, world);

    // 获取最有一个字符
    let len = s.len();
    let last = &s[len-1..len];
    println!("last : {}", last);

    // 获取完整String切片
    let slice1 = &s[..];
    let slice2 = &s[0..len];
    let slice3 = &s[..len];
    println!("{}-{}-{}", slice1, slice2, slice3);

    // 使用切片对中文字符串切片时需要非常注意: 中文在utf8中占用3个字节，英文字符占用1个字节;
    let s2 = "中国人";
    // let slice4 = &s2[0..2];  // 切片获取第1、2个字节的，所以会报错
    // byte index 2 is not a char boundary; it is inside '中' (bytes 0..3) of `中国人`'
    let slice4 = &s2[0..3];  
    println!("{}", slice4);

    let s3 = String::from("hello world");
    let frist_word = get_first_word(&s3);  // 动态字符串的不可变引用类型传给函数，并返回切片不可变引用类型
    // pub fn clear(&mut self) 需要接收 s自身的 可变借用
    // s.clear();  // 报错: error, 借用若有可变借用则无法再拥有不可变借用，因此clear需要清空变量String，需要可变借用;
    println!("frist_word: {}", frist_word);

    // 1.3 其他切片
    let a = [1, 2, 3, 4, 5];  
    // 整型切片
    let slice5 = &a[..];
    // assert_eq!(slice5, &[1, 2, 3, 4, 5, 6]);  报错，断言失败
    assert_eq!(slice5, &[1, 2, 3, 4, 5, ]);

    // 1.4 字符串字面量，类型是 &str
    // 字符串字面量是不可变的
    let s4 = "hello world";  // s4  ==   s4: &str
    let s5:&str = "hello world";
}

// 字符串切片的类型标识: &str , 因此可以传递给函数 &String 类型的参数，然后返回 &str的切片;
fn get_first_word(s: &String) -> &str{
    &s[..1]  // 返回第一个字符
}

/*
    1.5 什么是字符串:
        字符串是由字符组成的连续集合，Rust中的 ”字符是Unicode类型“，因此每个字节占据4个字节内存空间，但是
    字符串是UTF-8编码，也就是 字符串 中的 字符 所占的 字节数是变化的(1-4z之间)，有利于降低字符串所占用的空间;

        Rust语言中，基本类型中只有一种字符串类型: str, 通常是以引用类型出现 &str，也就是 字符串切片，除了基本类型，
    在标准库中，还有多种不同用途的字符串类型，其中最为广泛的是String类型;

        str类型是硬编码可执行文件，是无法被修改的，但是String则是一个可变字符串，具备所有权的UTF-8的编码字符串，
    当Rust用户提到字符串时，往往指的是 String 类型 和 &str 字符串切片类型, 这两种类型都是UTF-8编码; 

        Rust标准库中其他类型的字符串，如: OsString、OsStr、CsString、CsStr等.. 分别对应的是具有 所有权和被借用的变量;
*/ 

// 1.6 String 与 &str 的转换
fn string_change_str(){
    // &str转换为String
    // "hello world".to_string();

    let s = String::from("hello world!");
    //  String 就会变成 &str 
    say_hello(&s);  // 引用 String 就会变成 &str 
    say_hello(&s[..]);  // 引用 String 就会变成 &str 

    /*
        String.as_str()源码:
            #[stable(feature = "string_as_str", since = "1.7.0")]
            pub fn as_str(&self) -> &str {
                self
            }
    */
    say_hello(s.as_str());  // 隐式强制转换,通过方法转换
}
fn say_hello(s:&str){
    println!("s: {}", s);
}

// 1.7 字符串索引
fn str_index(){
    let s = String::from("tankjam");
    // 注意: 其他语言中通过索引获取字符串中某个字符是不会报错，但是Rust会报错
    // let t_str = s[0];  // Index<{integer}>` is not implemented for `String`
    let t_str = &s[0..1];  // 通过这种方式才能获取

    /*
        - 不能直接通过 索引 去引用某个字符的原因,Rust在字符内部的规则:
            1.字母形式，一个字符在UTF-8编码中占用1个字节
            let s = String::from("hello");  // 占用6个字节
            2.中文形式，一个字符在UTF-8编码中占用3个字节
            let s2 = String::from("你好呀"); // 占用9个字节
            
            - 若都取索引为0的字符，则中文无法获取，所以Rust就限制了，不能直接通过索引的方式获取，只能通过切片精确获取;
                s[0]  --> h
                s2[0] --> 报错

            - Rust不允许通过索引取字符串，因为索引操作，需要控制时间复杂度为 O(1), 然而对于String类型来说，无法保证
            这一点，因为Rust可能需要从0开始去遍历字符串来定位合法的字符;
    */
}

// 1.8 操作字符串 （String）
fn string_function(){
    // - 追加 Push
    let mut s = String::from("hello ");  // 定义可变的动态字符串
    // 1) 追加字符 push()
    s.push('t');  
    println!("push 追加字符: {}", s);

    // 2) 追加字符串 push_str()
    s.push_str("ank!"); 
    println!("push_str 追加字符串: {}", s);


    // - 插入 Insert
    let mut s2 = String::from("hllo !");
    // 1) 插入字符 insert(字符串索引, '字符');
    s2.insert(1, 'e');
    println!("insert在索引为1的位置插入字符: {}", s2);  // hello !

    // 2) 插入字符串 insert_str(字符串索引, "字符串");
    s2.insert_str(6, "rust");
    println!("insert在索引为6的位置插入字符串: {}", s2);  // hello rust!


    // - 替换 Replace、replacen、replace_range
    // 适用于 String 和 &str
    let string_repalce = String::from("I like rust, Learning rust is my favorite!");
    // 1) replace("原字符", "替换字符") : 接收 “两个参数”，替换所有参数中字符，会返回一个新的字符串，而不是原来的字符串;
    // String 中 rust 替换为 RUST
    let new_string_replace = string_repalce.replace("rust", "RUST");
    dbg!(new_string_replace);  // 宏: dbg!(new_string_replace)代替println!(new_string_replace)

    // 2) replacen("原字符", "替换字符", 替换个数) : 接收 “三个参数”，根据个数替换字符，会返回一个新的字符串，而不是原来的字符串;
    let new_string_replace2 = string_repalce.replacen("rust", "RUST", 1);
    dbg!(new_string_replace2);

    // 3) replace_range(索引切片, "替换的字符"): 通过迭代器的方式，直接操作原来的字符串，不会返回新的字符串，但是需要通过mut来修饰;
    let mut string_replace2 = String::from("I like rust!");
    string_replace2.replace_range(7..8, "R");  // 替换索引为7的字符
    dbg!(string_replace2);


    // - 删除 Delete (pop, remove, truncate, clear) 仅适用于String类型;
    // 1) pop(): 直接操作原字符串，若括号中没有参数，则取出最后一个字符，若字符串没有值则返回None
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();  // 去除字符串中最后一个字符，绑定给p1
    let p2 = string_pop.pop();  // 去除字符串中最后一个字符，绑定给p2
    dbg!(p1);
    dbg!(p2);
    // 剩下的原字符串，则没有的最后两个字符
    dbg!(string_pop);

    // 2) remove(): 直接操作原字符串，返回删除位置的字符串，只接收一个参数，表示媳妇起始索引位置。
    // 该方法按照字节来处理字符串，如果参数所给的位置不是合法的字符边界，则会发生错误
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占用 {} 个字节!",
        // std::mem::size_of_val查看 string类型转为 &str 后占用字节的个数
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字, 只能通过
    string_remove.remove(0);  // 接收的参数为删除字符的索引位置
    // 当在remove中指定索引位置进行删除，必须要注意，前面的字符串是否为中文，若是则要根据字符的长度来跳过中文字符
    // string_remove.remove(1);  // 报错，因为索引为1的字节在第一个中文字符内
    string_remove.remove(3);  // 不会报错，因为从 "试remove方法" 字符串中的字节索引3开始，也就是 r 字符;
    dbg!(string_remove);

    // 3) truncate(): 直接操作原字符串，无返回值，会按照参数作为字节的起始位置，往后删除所有的字符，若起始位置
    // 不合法（在中文的字节中），则会报错;
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);  // 从第二个中文字符的起始位置删除后面所有的字符
    dbg!(string_truncate);

    // 4) clear(): 清空字符串
    let mut string_clear = String::from("测试clear");
    string_clear.clear();
    dbg!(string_clear);


    // - 连接（Catenate）
    // 1）使用 + 或者 += 来实现字符串拼接，要求右边的参数必须为字符串的切片引用类型。调用 + 的时候，相当于调用了 std::string
    //  标准库中的 add() 方法，add方法中第二个参数是一个引用类型; + 和 += 都会返回一个新的字符串(), 无须mut修饰对原值修改;
    let string_hello = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_hello + &string_rust;  // 右边的值必须是切片引用类型，所以使用了&将String转为&str
    let mut result = result + "!";  // 需要对原有的result进行修改，将result设置为可变类型
    result += "!!!";
    println!("连接字符串: {}", result);

    let str1 = String::from("hello,");
    let str2 = String::from("world!");
    let str3 = str1 + &str2;  // str1的所有权会转移，str2是被引用, 后续str1不能再使用,此行代码结束，str1会被释放;
    assert_eq!(str3, "hello,world!");
    // println!("str1: {}", str1);  // 报错，str1所有权已经被转移了
    // 多个字符串之间拼接
    let ss1 = String::from("ss1");
    let ss2 = String::from("ss2");
    let ss3 = String::from("ss3");
    let str4 = ss1 + "-" + &ss2 + "-" + &ss3;
    println!("str4: {}", str4);

    // 2）format! 连接字符串: 适用于 String 和 &str，用法与print!相似，称之为格式化输出;
    let f1 = "hello";  // &str
    let f2 = String::from("rust");  // String
    let str5 = format!("{} {}!", f1, f2);
    println!("format!--> {}", str5);

}

// 1.9 字符串转义：可以通过转移的方式 \ 输出 ASCII 和 Unicode 字符;
fn string_transfer(){
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个unicode字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";  // 在字符串中使用双引号 \" \" -->  "字符串""中的双引号"
    println!("Unicode character: {} (U+211D) is called {}", unicode_codepoint, character_name);

    // 换行了也会保持之前的字符串格式  第一行结尾->\  第二行开头 <-  就能保持第一二行不会因为输入换行而换行
    let long_string = "String literals
                        can span multiple lines. 
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    // 在字符串中有转义符却想要保持原样
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以开字符串开头和结尾加#
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还有歧义，可以继续增加，没有限制 (包含所有特殊字符，比如："和#)
    let longer_delimier = r###"A string with "# in it. And even "###;
    println!("{}", longer_delimier);
}

// 1.10 操作UTF-8字符串
fn doing_utf8_str(){
    // 字符
    // chars方法遍历Unicode字符串 
    for c in "中国人".chars(){
        println!("{}", c);
    }

    // 字节
    // bytes()方法遍历 底层字节数组
    for b in "中国人".bytes(){
        println!("{}", b);
    }

    // 获取子串
    // 从UTF-8字符串中获取子串，标准库无法实现，需要通过utf8_slice库来实现
}

// 1.11 字符串深度剖析
fn str_question(){
    /*
    字符串深度剖析:
        问题1: String可变，字符串字面值str不可变
        答案1: 
            - 字符串字面值str 会在rust编译时就知道其内容，并硬编码进可执行文件中，性能非常快且高效，因为存在栈中;
            - String类型，为了能支持可变、可增长的文本，需要再堆上分配一块编译时未知大小的内存空间来存放内容;
                - 执行流程:
                    1.先向操作系统申请内存用于存放String对象;
                    2.在使用完成后，将内存释放，并归返操作系统;
        重点:
            在有 垃圾回收GC 的语言中，GC来负责标记并清除这些不再使用的内存对象，过程都是自动完成，无需开发者关心;
            在无 GC 的语言中，需要开发者手动去释放内存对象;

            Rust（安全+性能）,如果使用GC会牺牲性能，如果手动管理内存，会牺牲安全，所以相当一个非常经验的办法，
        在变量离开作用域后，就会自动释放其内存;

        其他语言需要手动使用 free 函数来释放内存，Rust会在 } 后面自动调用 drop 来实现内存释放
    */ 
    {
        let s = String::from("hello");  // 此处起始 s 有效
        // 使用s
    }   // 此处开始作用域已结束
        // s 不再有效，内存已被释放
}


// 1.12 字符串作业
use std::str;
use utf8_slice;
fn str_work(){
    // 1.
    let s: &str = "hello, world";
    println!("{}", s);

    // 2. Box<str> 转为 &str
    let _s2:Box<str> = "hello world!".into();

    // 3. 
    let mut s3 = String::from("");
    s3.push_str("hello, world");
    s3.push('!');
    assert_eq!(s3, "hello, world!");

    // 4.
    let mut s4 = String::from("hello");
    s4.push(','); 
    s4.push_str(" world");
    s4 += &"!".to_string();   // += 必须是要切片引用类型
    s4 += "!"; 
    println!("s4: {}", s4);

    // 5.
    let s5 = String::from("I like dogs");
    let ss5 = s5.replace("dogs", "cats");
    assert_eq!(ss5, "I like cats");

    // 6.
    let s6 = String::from("hello,");
    let ss6 = String::from(" world!");
    let sss6 = s6 + &ss6; // 自动调用 add(String, &str引用类型)
    assert_eq!(sss6, "hello, world!");
    // println!("{}", s6); s6的所有权被移除

    // 7. &str -> String
    // 方式一:
    // let s7 = "hello, world!";
    // greetings(s7.to_string());
    let s7 = String::from("hello, world!");
    greetings(s7);

    // 8. &str -> String
    let s8 = "hello, world!".to_string();
    let ss8 = String::from("hello, world!");
    let sss8:&str = s8.as_str();  // String -> &str
    let ssss8:&str = &ss8; 
    let sssss8:&str = &ss8[..]; 
    println!("{} - {} - {}", sss8, ssss8, sssss8);

    // 9.字符串转义
    // 10.

    // 字节字符串
    // 定义一个长度为21的字节字符串，也称之为字节数组
    let bytes_str:&[u8; 21] = b"this is a byte string"; 
    println!("A byte string: {:?}", bytes_str);  // {:?} 格式化输出 字节数组/字节字符串

    // 字节数组转义
    let bytez_escaped = b"\x52\x75\x73\x74 as bytes";
    println!("{:?}", bytez_escaped);
    // 但是不支持 Unicode 转义
    // let unicode_escaped = b"\u{211D} is not allowed";

    // raw string 
    let raw_bytestring = br"\u{211D} is not escaped here"; 
    println!("{:?}", raw_bytestring);

    // 将字节数组转成 str 类型可能会失败，可以做一个判断
    // 将 raw_bytestring 转成 utf8的str类型
    if let Ok(my_str) = str::from_utf8(raw_bytestring){
        println!("And the same as text: '{}'", my_str);
    }

    // 11.
    let s11 = String::from("hi,中国");
    let h = &s11[0..1];
    println!("{}", h);

    let h2 = &s11[3..6];
    println!("{}", h2);

    // 12.
    for c in "你好，中国".chars(){
        println!("{}", c);
    }

    // utf8_slice 使用
    let utf8_str = "The 🚀 goes to the 🌚!";
    let rocket = utf8_slice::slice(utf8_str, 4, 5);
    println!("{}", rocket);
}
fn greetings(s:String){
    println!("s7: {}", s);
}


 
/*
    2、元组
    元组是由多种类型组合而成，因此是复合类型，元组的长度是固定的，元组中的元素的顺序也是固定的;
*/
fn tup_main(){
    // 绑定元组
    let tup:(i32, f64, u8) = (500, 9.87, 1);
    println!("tup: {:?}", tup);

    // 模式匹配解构元组 （类似于py的解压赋值）
    let(x, y, z) = tup;
    println!("The x value is {}", x);

    // 使用  .  访问元组某个特定元素 (根据元组值的索引获取指定下标的元素)
    let tup2: (i32, f64, u8) = (9527, 9.527, 2);
    let num1 = tup2.0;
    let num2 = tup2.1;
    let num3 = tup2.2;
    println!("{} - {} - {}", num1, num2, num3);

    // 元组的使用示例
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("Ther length of '{}' is {}", s2, len);
}

// 计算长度
fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}   


/*
    3、结构体
    可以存放不同类型的元素，并且可以给元素命名;
*/
fn struct_main(){
    // 3.1 定义结构体

    /*
       3.2 创建结构体实例
       注意:
          1.初始化实例时，每个字段都需要进行初始化;
          2.初始化时字段的顺序可以是无序的;
    */ 
    let user1 = User{
        active: true,
        username: String::from("tank jam"),
        email: String::from("tankjam.com"),
        sign_in_count: 1,
    };

    // 3.3 访问结构体字段
    // 获取
    println!("{}", user1.email);
    // 修改 注意: 必须在定义阶段，声明为可变结构体，不能对某一个字段设置为可变
    // user1.email = String::from("xxx.com");
    // 

    // 3.4 简化结构体创建
    // 调用构建函数
    let user_obj = build_user(String::from("zhangquandan.com"), String::from("张全蛋"));

    // 3.5 结构体更新语法
    // 更新user_obj对象的email字段
    let user2 = User{
        email: String::from("tankjam9527.com"),
        ..user_obj // 将没有更新的字段全部传给user2
        // 只有username是将所有权转移给user2，active和sign_in_count是发生了拷贝（Copy特征）
        // 那么user_obj中只有username字段不可用，其他字段还是可以继续使用的;
    };
    println!("user_obj.active: {}", user_obj.active);
    // 报错
    // println!("user_obj.username: {}", user_obj.username);

    // 3.6 结构体内存排序
    let f1 = File{
        name: String::from("f1.txt"),
        data: Vec::new(),
    };
    let f1_name = &f1.name; // 将f1对象指针（引用类型）中的name字段 所有权 转移给了f1_name变量
    let f1_length = &f1.data.len(); // 将 f1 底层指针对应的数组长度，赋于 f1_length

    // println!("{:?}", f1);  // 因为f1中的name字段所有权已经被转移了
    println!("{} is {} bytes long", f1_name, f1_length);

    // 3.7 元组结构体
    // 结构体必须要有名称，但是结构体的字段可以没有名字，长得很像元组，因此称之为 "元组结构体";
    let black = Color(0, 0, 0);   // 通过结构体构造 px 是 黑色的对象
    let origin = Point(0, 0, 0);  // 通过结构体构造 px 是 橙色的对象

    // 3.8 单元结构体（Unit-like Struct）
    // 与基本类类型的 单元 类型很像，没有任何 “字段和属性”，但是它还有非常有用;
    // 若定义一个类型，不关心其类型的内容，只关心它的行为，建议使用 “单元结构体” 
    // 空属性结构体 --> 是需要绑定方法
    let subject = AlwaysEqual;
    // 绑定SomeTrait方法

    // 3.9 结构体数据的所有权
    // 在User结构体中，使用了String类型，而不是引用 &str 切片类型，因为想要让结构体拥有它所有的数据，
    // 而不是引用其他地方的数据，若想让结构体从其他对象借用数据，需要引入生命周期的概念，这样能确保结构体
    // 的作用范围比借用的数据作用域范围小;
    // let stu = Student{
    //     name: "张全蛋",
    //     email: "xxx.com",
    //     active: true,
    // };

    // 3.10 使用 #[derive(Debug)] 来打印结构体的信息
    // rust中打印结构体需要使用 #[derive(Debug)] 来进行标记，才能使用 println!("{:?}", obj);
    // 如果不加则会报错;
    let rect1 = Rectangle {
        width: 500,
        height: 250,
    };
    // 报错: ectangle` cannot be formatted with the default formatter
    // println!("{}", rect1);
    // 报错: Rectangle` cannot be formatted using `{:?}`
    // println!("{:?}", rect1);

    /*
        结构体 使用 #[derive(Debug)] 就不会报错:
        Rust默认不会提供Debug，为了实现Debug有两种方式:
            · 手动实现（定制化）
            · 使用derive派生实现（推荐，但有限制）
    */ 
    println!("{:?}", rect1);
    println!("{:#?}", rect1);  // 样式优化输出 :#?

    // 利用宏也可以输出结构体信息, 可以打印 文件名、行号等Debug信息、表达式的求值结果，还可以把表达式值的所有权返回;
    // dbg!     输出到标准错误输出  stderr
    // println! 输出到标准输出     stdout
    let scale = 2;
    let rect2 = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);  // 将 30 * scale 表达式的值返回
    /*
    [src/main.rs:619] 30 * scale = 60
    [src/main.rs:622] &rect2 = Rectangle {
        width: 60,
        height: 50,
    }
    */

    // 3.11 作业
    struct_work();
}

/*
- 定义结构体 (3.1)
    · 通过关键字 “struct” 定义
    · 一个清晰明确的结构体 “名称”
    · 几个有名字的结构体 “字段”
*/ 
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count:u64,
}

// 构建结构体 (3.4)
fn build_user(email: String, username: String) -> User {
    // 传统构建，有点鸡肋
    // User{
    //     email: email,
    //     username:username,
    //     active: true,
    //     sign_in_count:1,
    // }

    // 简化结构体
    User{
        email,
        username,
        active: true,
        sign_in_count:1,
    }

}

// 文件结构体（3.6）
struct File{
    name: String,
    data: Vec<u8>,  // Vec<u8> 整型动态数组
}

// 元组结构体 (3.7)
struct Color(i32, i32, i32);  
struct Point(i32, i32, i32);

// 单元结构体（3.8）
struct AlwaysEqual;
// 结构体AlwaysEqual绑定SomeTrait方法
// impl SomeTrait for AlwaysEqual{}

// 结构体数据的所有权（3.9）
// // 结构体引用其他对象
// struct Student {
//     // error: missing lifetime specifier
//     name: &str, //引用类型
//     email: &str,
//     active: bool,
// }

// 使用 #[derive(Debug)] 来打印结构体的信息 (3.10)
#[derive(Debug)] 
struct Rectangle{
    width: u32,
    height: u32,
}

// 结构体作业 (3.11)
fn struct_work(){
    // 1.
    let v = Color2(0, 127, 255);
    check_color(v);

    // 2.
    let f = File3 {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };
    let _name = f.name;
    // ONLY modify this line
    println!("{}, {}",_name, f.data);

    // 3. 没看懂
    // let num = Some(4);
    // let split = 5;
    // match num {
    //     Some(x) __ => assert!(x < split),
    //     Some(x) => assert!(x >= split),
    //     None => (),
    // }
}
// 定义元组结构体
struct Color2(i32, i32, i32);
struct Point2(i32, i32, i32);
fn check_color(obj: Color2){
    let x = obj.0;
    assert_eq!(x, 0);
    assert_eq!(obj.1, 127); // 元组结构体，需要通过索引来获取对应的元素
    assert_eq!(obj.2, 255); 
}

#[derive(Debug)]
struct File3 {
    name: String,
    data: String,
}


// - 枚举
// - 数组


fn main() {
    // 复合类型初体验
    example();

    // 1.字符串
    string_main();

    // 2.元组
    tup_main();

    // 3.结构体
    struct_main();
    
}
