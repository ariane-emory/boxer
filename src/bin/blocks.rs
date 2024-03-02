#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]

use boxer::block;
use boxer::block::*;

use std::cell::RefCell;
use std::io::{self};
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
type RcRcSteppable = Rc<RefCell<dyn Steppable>>;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn add_to_rcrc_steppable_vec<T: 'static + Steppable>(blocks: &mut Vec<RcRcSteppable>, item: &Rc<RefCell<T>>) {
  let steppable_item: RcRcSteppable = item.clone() as Rc<RefCell<dyn Steppable>>;
  blocks.push(steppable_item);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn new_rcrc<T>(item: T) -> Rc<RefCell<T>> { Rc::new(RefCell::new(item)) }

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn render(char: u8, char2: u8, signal: usize, width: usize) {
  let mut printed = 0;
  let quarterways = width >> 2;
  let mut next_div = quarterways;

  print!("|");
  for _ in 0..signal {
    if printed == next_div {
      print!("|");
      next_div = next_div + quarterways;
    }
    print!("{}", char as char);
    printed = printed + 1;
  }
  for _ in 0..(width - signal) {
    if printed == next_div {
      print!("|");
      next_div = next_div + quarterways;
    }
    print!("{}", char2 as char);
    printed = printed + 1;
  }
  println!("|");
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  let one = Value::new(1);
  let never = Value::new(false);
  let sixteen = Value::new(16);
  let max = Value::new(128);
  let ifour = Value::<isize>::new(4);
  let imax = Value::<isize>::new(max.output_value() as isize);
  let ithree = Value::<isize>::new(3);
  let itwo = Value::<isize>::new(2);
  let izero = Value::new(0);
  let clock = SquareWave::new(one.output());

  //loop {
  // {
  //   let mut counter_reset = Feedback::new();
  //   let counter_max = Value::new(64);
  //   let mut counter = UpCounter::new(clock.output(), counter_reset.output(), counter_max.output());
  //   let mut add = block::Add::new(counter.output(), one.output());
  //   let two = Value::new(2);
  //   let mut div = Div::new(add.output(), two.output());
  //   let mut square = SquareWave::new(div.output());
  //   let zero = Value::new(0);
  //   let mut select = Select::new(square.output(), zero.output(), max.output());

  //   counter_reset.set_input(&counter.at_max());

  //   for _ in 0..511 {
  //     clock.step();
  //     counter.step();
  //     add.step();
  //     div.step();
  //     square.step();
  //     select.step();
  //     counter_reset.step();

  //     // println!("");
  //     // println!("counter input:  {}", clock.output().borrow().read());
  //     // println!("counter output: {}", counter.output().borrow().read());
  //     // println!("add output:     {}", add.output().borrow().read());
  //     // println!("square period:  {}", square.period().borrow().read());
  //     // println!("square output:  {}", square.output().borrow().read());
  //     // println!("select output:  {}", select.output().borrow().read());

  //     render(b'x', b'-', select.output_value(), max.output_value());
  //   }
  // }

  {
    let one = new_rcrc(Value::new(1));
    let max = new_rcrc(Value::new(128));
    let clock = new_rcrc(SquareWave::new(one.borrow_mut().output()));
    let counter_reset = new_rcrc(Feedback::new());
    let counter = new_rcrc(UpCounter::new(clock.borrow_mut().output(), &counter_reset.borrow_mut().output(), max.borrow_mut().output()));
    let sr_set = new_rcrc(Feedback::new());
    let sr_reset = new_rcrc(Feedback::new());
    let sr = new_rcrc(SRLatch::new(sr_set.borrow_mut().output(), sr_reset.borrow_mut().output()));
    let not_latched = new_rcrc(Not::new(sr.borrow_mut().output()));
    let counter_at_max_and_latched = new_rcrc(And::new(counter.borrow_mut().at_max(), sr.borrow_mut().output()));
    let counter_at_max_and_not_latched = new_rcrc(And::new(counter.borrow_mut().at_max(), not_latched.borrow_mut().output()));
    let sub = new_rcrc(Sub::new(max.borrow_mut().output(), counter.borrow_mut().output()));
    let select = new_rcrc(Select::new(sr.borrow_mut().output(), counter.borrow_mut().output(), sub.borrow_mut().output()));

    counter_reset.borrow_mut().set_input(&counter.borrow_mut().at_max());
    sr_set.borrow_mut().set_input(&counter_at_max_and_not_latched.borrow_mut().output());
    sr_reset.borrow_mut().set_input(&counter_at_max_and_latched.borrow_mut().output());

    for _ in 0..511 {
      // println!("");

      clock.step();
      counter.step();
      not_latched.step();
      counter_at_max_and_latched.step();
      counter_at_max_and_not_latched.step();
      sr.step();
      sub.step();
      select.step();
      counter_reset.step();
      sr_set.step();
      sr_reset.step();

      // println!("counter:             {}", counter.output_value());
      // println!("counter.at_max:      {}", counter.at_max.borrow().read());
      // println!("counter reset:       {}", counter_reset.output_value());
      // println!("sr_reset:            {}", sr_reset.output_value());
      // println!("sr:                  {}", sr.output_value());
      // println!("sub:                 {}", sub.output_value());
      // println!("select:              {}", select.output_value());
      // println!("not_latched:         {}", not_latched.output_value());
      // println!("max_and_latched:     {}", max_and_latched.output_value());
      // println!("max_and_not_latched: {}", max_and_not_latched.output_value());

      render(b'x', b'-', select.output_value(), max.output_value());
    }
  }
  {
    let clock = new_rcrc(SquareWave::new(one.output()));
    let square = new_rcrc(SquareWave::new(sixteen.output()));
    let select = new_rcrc(Select::new(square.borrow_mut().output(), izero.output(), imax.output()));
    let held_value = new_rcrc(Feedback::<isize>::new());
    let div_held_value_by_itwo = new_rcrc(Div::<isize>::new(held_value.borrow_mut().output(), itwo.output()));
    let div_new_input_by_itwo = new_rcrc(Div::<isize>::new(select.borrow_mut().output(), itwo.output()));
    let add = new_rcrc(Add::new(div_held_value_by_itwo.borrow_mut().output(), div_new_input_by_itwo.borrow_mut().output()));
    let sample_and_hold = new_rcrc(SampleAndHold::new(add.borrow_mut().output(), clock.borrow_mut().output(), never.output()));

    held_value.borrow_mut().set_input(&sample_and_hold.borrow_mut().output());

    let mut blocks: Vec<RcRcSteppable> = Vec::new();
    add_to_rcrc_steppable_vec(&mut blocks, &clock);
    add_to_rcrc_steppable_vec(&mut blocks, &square);
    add_to_rcrc_steppable_vec(&mut blocks, &select);
    add_to_rcrc_steppable_vec(&mut blocks, &held_value);
    add_to_rcrc_steppable_vec(&mut blocks, &div_held_value_by_itwo);
    add_to_rcrc_steppable_vec(&mut blocks, &div_new_input_by_itwo);
    add_to_rcrc_steppable_vec(&mut blocks, &add);
    add_to_rcrc_steppable_vec(&mut blocks, &sample_and_hold);

    for _ in 0..511 {
      blocks.iter_mut().for_each(|b| b.borrow_mut().step());
      render(b'x', b'-', add.borrow_mut().output_value() as usize, imax.output_value() as usize);
    }
  }
  Ok(())
}
