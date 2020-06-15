use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("猜数字");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("生成的随机数字是：{}", secret_number);

  println!("请输入一个数字：");

  let mut guess = String::new();

  io::stdin().read_line(&mut guess)
      .expect("失败，未能读输入");

  let guess: u32 = guess.trim().parse()
      .expect("请输入一个正确的数字");

  println!("你输入的是: {}", guess);
}
