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


////////////////////////////////////////////////////////////////////////////////////////////////////
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
  let mut clock = SquareWave::new(one.output());

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

  // {
  //   let mut counter_reset = Feedback::new();
  //   let mut counter = UpCounter::new(clock.output(), &counter_reset.output(), max.output());
  //   let mut sr_set = Feedback::new();
  //   let mut sr_reset = Feedback::new();
  //   let mut sr = SRLatch::new(sr_set.output(), sr_reset.output());
  //   let mut not_latched = Not::new(sr.output());
  //   let mut counter_at_max_and_latched = And::new(counter.at_max(), sr.output());
  //   let mut counter_at_max_and_not_latched = And::new(counter.at_max(), not_latched.output());
  //   let mut sub = Sub::new(max.output(), counter.output());
  //   let mut select = Select::new(sr.output(), counter.output(), sub.output());

  //   counter_reset.set_input(&counter.at_max());
  //   sr_set.set_input(&counter_at_max_and_not_latched.output());
  //   sr_reset.set_input(&counter_at_max_and_latched.output());

  //   for _ in 0..511 {
  //     // println!("");

  //     clock.step();
  //     counter.step();
  //     not_latched.step();
  //     counter_at_max_and_latched.step();
  //     counter_at_max_and_not_latched.step();
  //     sr.step();
  //     sub.step();
  //     select.step();
  //     counter_reset.step();
  //     sr_set.step();
  //     sr_reset.step();

  //     // println!("counter:             {}", counter.output_value());
  //     // println!("counter.at_max:      {}", counter.at_max.borrow().read());
  //     // println!("counter reset:       {}", counter_reset.output_value());
  //     // println!("sr_reset:            {}", sr_reset.output_value());
  //     // println!("sr:                  {}", sr.output_value());
  //     // println!("sub:                 {}", sub.output_value());
  //     // println!("select:              {}", select.output_value());
  //     // println!("not_latched:         {}", not_latched.output_value());
  //     // println!("max_and_latched:     {}", max_and_latched.output_value());
  //     // println!("max_and_not_latched: {}", max_and_not_latched.output_value());

  //     render(b'x', b'-', select.output_value(), max.output_value());
  //   }
  // }

  {
    let mut clock = Rc::new(RefCell::new(SquareWave::new(one.output())));
    let mut square = Rc::new(RefCell::new(SquareWave::new(sixteen.output())));
    let mut select = Rc::new(RefCell::new(Select::new(square.borrow_mut().output(), izero.output(), imax.output())));
    let mut held_value = Rc::new(RefCell::new(Feedback::<isize>::new()));
    let mut div_held_value_by_itwo = Rc::new(RefCell::new(Div::<isize>::new(held_value.borrow_mut().output(), itwo.output())));
    let mut div_new_input_by_itwo = Rc::new(RefCell::new(Div::<isize>::new(select.borrow_mut().output(), itwo.output())));
    let mut add = Rc::new(RefCell::new(Add::new(div_held_value_by_itwo.borrow_mut().output(), div_new_input_by_itwo.borrow_mut().output())));
    let mut sample_and_hold = Rc::new(RefCell::new(SampleAndHold::new(add.borrow_mut().output(), clock.borrow_mut().output(), never.output())));
    held_value.borrow_mut().set_input(&sample_and_hold.borrow_mut().output());

    // let steppable_obj: Rc<RefCell<dyn Steppable>> =
    let mut blocks: Vec<Rc<RefCell<dyn Steppable>>> = Vec::new();
    let steppable_obj: Rc<RefCell<dyn Steppable>> = clock.clone();
    blocks.push(steppable_obj);
    let steppable_obj: Rc<RefCell<dyn Steppable>> = square.clone();
    blocks.push(steppable_obj);
    let steppable_obj: Rc<RefCell<dyn Steppable>> = select.clone();
    blocks.push(steppable_obj);
    let steppable_obj: Rc<RefCell<dyn Steppable>> = held_value.clone();
    blocks.push(steppable_obj);
    let steppable_obj: Rc<RefCell<dyn Steppable>> = div_held_value_by_itwo.clone();
    blocks.push(steppable_obj);
    let steppable_obj: Rc<RefCell<dyn Steppable>> = div_new_input_by_itwo.clone();
    blocks.push(steppable_obj);
    let steppable_obj: Rc<RefCell<dyn Steppable>> = add.clone();
    blocks.push(steppable_obj);
    let steppable_obj: Rc<RefCell<dyn Steppable>> = sample_and_hold.clone();
    blocks.push(steppable_obj);

    for _ in 0..511 {
      //blocks.iter_mut().for_each(|b| b.step());
      clock.borrow_mut().step();
      square.borrow_mut().step();
      select.borrow_mut().step();
      held_value.borrow_mut().step();
      div_new_input_by_itwo.borrow_mut().step();
      div_held_value_by_itwo.borrow_mut().step();
      add.borrow_mut().step();
      sample_and_hold.borrow_mut().step();

      // println!("");
      // println!("counter input:  {}", clock.output_value());
      // println!("counter output: {}", counter.output_value());
      // println!("add output:     {}", add.output_value());
      // println!("square period:  {}", square.period.output_value());
      // println!("square output:  {}", square.output_value());
      // println!("select output:  {}", select.output_value());
      // println!("s&h output:  {}", sample_and_hold.output_value());
      //render(b'x', b'-', select.output(), imax.output());

      render(b'x', b'-', add.borrow_mut().output_value() as usize, imax.output_value() as usize);
    }
  }
  Ok(())
}
