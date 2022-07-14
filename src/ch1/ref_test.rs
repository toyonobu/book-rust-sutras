fn main()
{
  let mut v=10;

  println!("変更前：{}", v);
  set_value(&mut v);
  println!("変更後：{}", v);
}

fn set_value(arg: &mut u32)
{
  *arg = 100;
}
