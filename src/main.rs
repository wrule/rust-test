use std::io;
use rand::Rng;

fn main() {
  println!("猜数字");

  println!("请输入一个数字：");

  let mut guess = String::new();

  io::stdin().read_line(&mut guess)
      .expect("失败，未能读输入");

  println!("你输入的是: {}", guess);

  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("{}", secret_number);
}
