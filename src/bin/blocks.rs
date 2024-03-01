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
  let mut adder = block::MathAdd::new(left.output(), left.output());
  let mut subber = block::MathSub::new(twenty.output(), adder.output());

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

  let counter_input = Value::new(false);
  let counter_reset = block::Value::new(false);
  let counter_max = Value::new(100);
  let mut counter = Counter::new(
    counter_input.output(),
    counter_reset.output(),
    counter_max.output(),
  );

  for _ in 0..10 {
    counter.step();
    println!("counter_input: {}", counter_input.output.borrow().read());
    println!("Counter: {}", counter.output.borrow().read());

    let counter_is_at_max = *counter.at_max.borrow().read();

    if counter_is_at_max {
      counter_input
        .output
        .borrow_mut()
        .set(!*counter_input.output.borrow().read());
    }

    let counter_input_val = *counter_input.output.borrow().read();
    counter_input.output.borrow_mut().set(!counter_input_val);
  }


  Ok(())
}
