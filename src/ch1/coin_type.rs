fn main()
{
  // price調度になる支払い方を全探索する
  let price:i64 = 3950;

  let count500: i64 = 10;
  let count100: i64 = 3;
  let count50: i64  = 10;

  for i500 in 0..(count500+1) {
    for i100 in 0..(count100+1) {
      for i50 in 0..(count50+1) {
        let total: i64 = 50 * i50 + 100*i100 + 500*i500;
        if total == price {
          println!("500円x{} + 100円x{} + 50円x{} = {}円", i500, i100, i50, total);
        }
      }
    }
  }
}
