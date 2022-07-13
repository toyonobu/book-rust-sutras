fn encrypt(text: &str, shift: i16) -> String
{
  let code_a = 'A' as i16;
  let code_z = 'Z' as i16;
  let mut result = String::new();

  for ch in text.chars() {
    let mut code = ch as i16;
    if code_a<=code && code<=code_z {
      // shiftが負の場合も考慮して+26が必要
      code = ((code-code_a) + shift +26) % 26 + code_a;
    }
    result.push((code as u8) as char);
  }
  return result;
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