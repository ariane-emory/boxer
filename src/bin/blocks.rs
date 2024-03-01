// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::block;
use boxer::block::*;
use std::io::{self};

fn render(char: u8, signal: &Signal<usize>) {
  for _ in 0..*signal.borrow().read() {
    print!("{}", char as char);
  }
  println!("");
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  // loop {
  // let left = block::Value::new(1);
  // let twenty = block::Value::new(20);
  // let mut add = block::MathAdd::new(left.output(), left.output());
  // let mut subber = block::MathSub::new(twenty.output(), add.output());

  // println!("Add: {}", add.output().borrow().read());
  // println!("Subber: {}", subber.output().borrow().read());

  // add.step();
  // subber.step();

  // println!("Add: {}", add.output().borrow().read());
  // println!("Subber: {}", subber.output().borrow().read());

  // left.output.borrow_mut().set(8);

  // add.step();
  // subber.step();

  // println!("Add: {}", add.output().borrow().read());
  // println!("Subber: {}", subber.output().borrow().read());
  // println!("");

  let one = Value::new(1);
  let mut fast_square = SquareWave::new(one.output());

  if false {
    let counter_reset = block::Value::new(false);
    let counter_max = Value::new(40);
    let mut counter = Counter::new(
      fast_square.output(),
      counter_reset.output(),
      counter_max.output(),
    );

    let mut add = block::MathAdd::new(counter.output(), one.output());

    let two = Value::new(2);
    let mut div = MathDiv::new(add.output(), two.output());

    let mut square = SquareWave::new(div.output());

    let max = Value::new(128);
    let zero = Value::new(0);
    let mut select = Select::new(square.output(), zero.output(), max.output());

    for _x in 0..255 {
      fast_square.step();
      counter.step();
      add.step();
      div.step();
      square.step();
      select.step();

      if *counter.at_max.borrow().read() {
        counter_reset.output.borrow_mut().set(true);
        counter.step();
        counter_reset.output.borrow_mut().set(false);
      }

      // println!("");
      // println!("counter input:  {}", fast_square.output.borrow().read());
      // println!("counter output: {}", counter.output().borrow().read());
      // println!("add output:     {}", add.output().borrow().read());
      // println!("square period:  {}", square.period.borrow().read());
      // println!("square output:  {}", square.output().borrow().read());
      // println!("select output:  {}", select.output().borrow().read());

      render(b'x', select.output());

      // let counter_input_val = *counter_input.output().borrow().read();
      // counter_input.output().borrow_mut().set(!counter_input_val);
    }
  }

  {
    let mut counter_reset = block::Jump::new();
    let counter_max = Value::new(40);
    let mut counter = Counter::new(
      fast_square.output(),
      counter_reset.output(),
      counter_max.output(),
    );

    let mut eql = Equal::new(counter.output(), counter_max.output());

    let clone = eql.output().clone();
    counter_reset.set_input(clone.clone());

    for _ in 0..255 {
      counter_reset.step();
      fast_square.step();
      counter.step();
      eql.step();

      render(b'x', counter.output());

      // println!(
      //   "{}",
      //   if *eql.output().borrow().read() {
      //     "YES"
      //   } else {
      //     "no"
      //   }
      // );

      // println!("{}", if *clone.borrow().read() { "YES" } else { "no" });
    }
  }

  Ok(())
}
