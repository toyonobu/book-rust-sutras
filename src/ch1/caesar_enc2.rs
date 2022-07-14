fn encrypt(text: &str, shift: i16) -> String
{
  let a = 'A' as i16;
  let is_az = |c| 'A'<=c && c<='Z';
 
  let conv = |c| ((((c-a) + shift+26)%26 + a) as u8) as char;
  let encl = |c| if is_az(c) {conv(c as i16)} else {c};

  return text.chars().map(|c| encl(c)).collect();
}

fn decrypt(text: &str, shift: i16) -> String
{
  return encrypt(text, -shift)
}

fn main()
{
  let raw: &str = "I LOVE YOU";
  let key = 3;
 
  let enc = encrypt(&raw, key);
  let dec = decrypt(&enc, key);

  println!("{} => {} => {}", raw, enc, dec);
}