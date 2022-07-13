fn main()
{
  // price調度になる支払い方を全探索する
  let price=3950;

  for i500 in 0..11 {
    for i100 in 0..4 {
      for i50 in 0..11 {
        let total = 50 * i50 + 100*i100 + 500*i500;
        if total == price {
          println!("500円x{} + 100円x{} + 50円x{} = {}円", i500, i100, i50, total);
        }
      }
    }
  }
}
