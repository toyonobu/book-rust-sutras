fn main() {
  let s = "3.1415";
  let num = s.trim().parse::<f64>().expect("変換に失敗");
  println!("{:.2}", num);
}
