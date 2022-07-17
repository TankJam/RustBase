/*
    模式匹配
        经常用于在函数式变成里，用于为复杂的类型系统提供一个轻松解构的能力;
        - Option枚举的进一步处理
        - match替代else if的多重分支适用
*/ 
// match
/*
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式
};
*/
enum Direction{
    East,
    West,
    North,
    South,
}

fn match_demo(){
    // 通过枚举选择South
    let dire = Direction::South;
    // 通过match dire去匹配Direction的所有成员
    match dire {
        // 如果是East，打印East
        Direction::East => println!("East"),
        // 如果是North 或者  South， 打印 South or North
        Direction::North | Direction::South => {
            println!("South or North");
        },
        // 如果前面匹配失败，默认打印 West
        _ => println!("West"),
    };
}

fn main() {
    
    // 1.match 和 if else
    // match模式匹配中最常用的一种
    match_demo();

    // 2.解构Option
    // 3.模式适用场景
    // 4.全模式列表
}
