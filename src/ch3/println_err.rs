fn main() {
    let s = "気持ちよく与えて豊かになる人がいる".to_string();
    echo(s); // ここで所有権移動
    println!("{}", s); // sはすでに空
}

fn echo(s: String) {
    println!("{}", s);
}
