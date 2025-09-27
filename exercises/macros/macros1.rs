// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 宏调用必须使用 宏名称!() 的形式
    my_macro!();
}
