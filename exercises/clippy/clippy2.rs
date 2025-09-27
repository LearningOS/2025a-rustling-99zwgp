// clippy2.rs
//
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        //直接检查 option 是否为 Some 变体，并将其中的值绑定到 x，而不需要使用 if let 语句。
        res += x;
    }
    println!("{}", res);
}
