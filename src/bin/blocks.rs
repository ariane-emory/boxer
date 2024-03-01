// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::block;
use boxer::block::*;
use std::io::{self};

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  // loop {
  let left = block::Value::new(1);
  let twenty = block::Value::new(20);
  let mut adder = block::MathAdd::new(&left.output, &left.output);
  let mut subber = block::MathSub::new(&twenty.output, &adder.output);

  println!("Adder: {}", adder.output.borrow().read());
  println!("Subber: {}", subber.output.borrow().read());

  adder.step();
  subber.step();

  println!("Adder: {}", adder.output.borrow().read());
  println!("Subber: {}", subber.output.borrow().read());

  left.output.borrow_mut().set(8);

  adder.step();
  subber.step();

  println!("Adder: {}", adder.output.borrow().read());
  println!("Subber: {}", subber.output.borrow().read());

  let flip = Value::new(false);
  let max = Value::new(100);
  let mut counter = Counter::new(&flip.output, &max.output);

  for _ in 0..10 {
    counter.step();
    println!("flip: {}", flip.output.borrow().read());
    println!("Counter: {}", counter.output.borrow().read());

    let flip_val = *flip.output.borrow().read();
    flip.output.borrow_mut().set(!flip_val);
  }


  Ok(())
}
