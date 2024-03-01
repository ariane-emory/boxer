// #![allow(unreachable_code)]
#![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::block;
use boxer::block::*;
use std::io::{self};

fn render(char: u8, char2: u8, signal: &Signal<usize>, width: &Signal<usize>) {
  for _ in 0..*signal.borrow().read() {
    print!("{}", char as char);
  }
  for _ in 0..(*width.borrow().read() - *signal.borrow().read()) {
    print!("{}", char2 as char);
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
  let mut clock = SquareWave::new(one.output());

  if false {
    let counter_reset = block::Value::new(false);
    let counter_max = Value::new(40);
    let mut counter = Counter::new(clock.output(), counter_reset.output(), counter_max.output());

    let mut add = block::MathAdd::new(counter.output(), one.output());

    let two = Value::new(2);
    let mut div = MathDiv::new(add.output(), two.output());

    let mut square = SquareWave::new(div.output());

    let max = Value::new(128);
    let zero = Value::new(0);
    let mut select = Select::new(square.output(), zero.output(), max.output());

    for _x in 0..255 {
      clock.step();
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
      // println!("counter input:  {}", clock.output.borrow().read());
      // println!("counter output: {}", counter.output().borrow().read());
      // println!("add output:     {}", add.output().borrow().read());
      // println!("square period:  {}", square.period.borrow().read());
      // println!("square output:  {}", square.output().borrow().read());
      // println!("select output:  {}", select.output().borrow().read());

      render(b'x', b'-', select.output(), counter_max.output());

      // let counter_input_val = *counter_input.output().borrow().read();
      // counter_input.output().borrow_mut().set(!counter_input_val);
    }
  }

  {
    let counter_max = Value::new(40);
    let mut counter_reset = Jump::new();
    let mut counter = Counter::new(clock.output(), counter_reset.output(), counter_max.output());

    let mut sr_set = Jump::new();
    let mut sr_reset = Jump::new();
    let mut sr = SRLatch::new(sr_set.output(), sr_reset.output());

    let mut sub = MathSub::new(counter_max.output(), counter.output());
    // let mut select = Select::new(delay2.output(), counter.output(), sub.output());
    let mut select = Select::new(sr.output(), counter.output(), sub.output());

    let mut not_latched = LogicNot::new(sr.output());
    let mut max_and_latched = LogicAnd::new(&counter.at_max, sr.output());
    let mut max_and_not_latched = LogicAnd::new(&counter.at_max, not_latched.output());

    let mut delay = UnitDelay::new(max_and_not_latched.output());

    let mut counter_reset_delay = UnitDelay::new(&counter.at_max);

    counter_reset.input = Some(counter_reset_delay.output().clone());
    sr_set.input = Some(max_and_not_latched.output().clone());
    sr_reset.input = Some(max_and_latched.output().clone());

    for _ in 0..1024 {
      // println!("");

      clock.step();
      counter.step();
      not_latched.step();
      max_and_latched.step();
      max_and_not_latched.step();
      delay.step();
      sr.step();
      sub.step();
      select.step();

      counter_reset_delay.step();
      counter_reset.step();
      sr_set.step();
      sr_reset.step();

      // println!("counter:             {}", counter.output().borrow().read());
      // println!("counter.at_max:      {}", counter.at_max.borrow().read());
      // println!("counter reset:       {}", counter_reset.output().borrow().read());
      // println!("sr_reset:            {}", sr_reset.output().borrow().read());
      // println!("sr:                  {}", sr.output().borrow().read());
      // println!("sub:                 {}", sub.output().borrow().read());
      // println!("select:              {}", select.output().borrow().read());
      // println!("not_latched:         {}", not_latched.output().borrow().read());
      // println!("max_and_latched:     {}", max_and_latched.output().borrow().read());
      // println!("max_and_not_latched: {}", max_and_not_latched.output().borrow().read());
      render(b'x', b'-', select.output(), counter_max.output());
    }
  }

  Ok(())
}
