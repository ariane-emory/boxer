//#![allow(unreachable_code)]
//#![allow(unused_variables)]
//#![allow(unused_imports)]
//#![allow(unused_mut)]
//#![allow(dead_code)]

use boxer::block::*;
use std::io::{self};

////////////////////////////////////////////////////////////////////////////////
const MAX: usize = 1 << 6;
const STEPS: usize = MAX << 2;

////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  {
    println!("\nSawtooth:");

    let one = Value::new(1).as_rcrc();
    let max = Value::new(MAX).as_rcrc();
    let clock = SquareWave::new(one.borrow_mut().output()).as_rcrc();
    let ctr_reset = Feedback::new().as_rcrc();
    let ctr_max = Value::new(MAX).as_rcrc();
    let ctr = UpCounter::new(
      clock.borrow_mut().output(),
      ctr_reset.borrow_mut().output(),
      ctr_max.borrow_mut().output(),
    )
    .as_rcrc();

    ctr_reset.borrow_mut().set_input(&ctr.borrow_mut().at_max());

    let mut blocks: Vec<RcRcSteppable> = Vec::new();
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &one);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &max);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &clock);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_reset);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_max);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr);

    for _ in 0..STEPS {
      blocks.iter().for_each(|b| b.borrow_mut().step());
      render(
        b'x',
        b'-',
        ctr.borrow_mut().output_value(),
        max.borrow_mut().output_value(),
      );
    }
  }

  {
    println!("\nSawtooth PWM:");

    let one = Value::new(1).as_rcrc();
    let max = Value::new(MAX).as_rcrc();
    let clock = SquareWave::new(one.borrow_mut().output()).as_rcrc();
    let ctr_reset = Feedback::new().as_rcrc();
    let ctr_max = Value::new(MAX).as_rcrc();
    let ctr = UpCounter::new(
      clock.borrow_mut().output(),
      ctr_reset.borrow_mut().output(),
      ctr_max.borrow_mut().output(),
    )
    .as_rcrc();
    let add =
      Add::new(ctr.borrow_mut().output(), one.borrow_mut().output()).as_rcrc();
    let two = Value::new(2).as_rcrc();
    let div =
      Div::new(add.borrow_mut().output(), two.borrow_mut().output()).as_rcrc();
    let square = SquareWave::new(div.borrow_mut().output()).as_rcrc();
    let zero = Value::new(0).as_rcrc();
    let select = Select::new(
      square.borrow_mut().output(),
      zero.borrow_mut().output(),
      max.borrow_mut().output(),
    )
    .as_rcrc();

    ctr_reset.borrow_mut().set_input(&ctr.borrow_mut().at_max());

    let mut blocks: Vec<RcRcSteppable> = Vec::new();
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &zero);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &one);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &max);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &two);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &clock);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_reset);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_max);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &add);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &div);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &square);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &select);

    for _ in 0..STEPS {
      blocks.iter().for_each(|b| b.borrow_mut().step());
      render(
        b'x',
        b'-',
        select.borrow_mut().output_value(),
        max.borrow_mut().output_value(),
      );
    }
  }

  {
    println!("\nTriangle:");

    let max = Value::new(MAX).as_rcrc();
    let one = Value::new(1).as_rcrc();
    let clock = SquareWave::new(one.borrow_mut().output()).as_rcrc();
    let ctr_reset = Feedback::new().as_rcrc();
    let ctr = UpCounter::new(
      clock.borrow_mut().output(),
      &ctr_reset.borrow_mut().output(),
      max.borrow_mut().output(),
    )
    .as_rcrc();
    let sr_set = Feedback::new().as_rcrc();
    let sr_reset = Feedback::new().as_rcrc();
    let sr = SRLatch::new(
      sr_set.borrow_mut().output(),
      sr_reset.borrow_mut().output(),
    )
    .as_rcrc();
    let not_latched = Not::new(sr.borrow_mut().output()).as_rcrc();
    let ctr_at_max_and_latched =
      And::new(ctr.borrow_mut().at_max(), sr.borrow_mut().output()).as_rcrc();
    let ctr_at_max_and_not_latched =
      And::new(ctr.borrow_mut().at_max(), not_latched.borrow_mut().output())
        .as_rcrc();
    let sub =
      Sub::new(max.borrow_mut().output(), ctr.borrow_mut().output()).as_rcrc();
    let select = Select::new(
      sr.borrow_mut().output(),
      ctr.borrow_mut().output(),
      sub.borrow_mut().output(),
    )
    .as_rcrc();

    ctr_reset.borrow_mut().set_input(&ctr.borrow_mut().at_max());
    sr_set
      .borrow_mut()
      .set_input(&ctr_at_max_and_not_latched.borrow_mut().output());
    sr_reset
      .borrow_mut()
      .set_input(&ctr_at_max_and_latched.borrow_mut().output());

    let mut blocks: Vec<RcRcSteppable> = Vec::new();
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &izero);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &max);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &one);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &clock);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_reset);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &sr_set);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &sr_reset);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &sr);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &not_latched);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_at_max_and_latched);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_at_max_and_not_latched);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &sub);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &select);

    for _ in 0..STEPS {
      blocks.iter().for_each(|b| b.borrow_mut().step());
      render(
        b'x',
        b'-',
        select.borrow_mut().output_value(),
        max.borrow_mut().output_value(),
      );
    }
  }

  {
    println!("\nSharktooth:");

    let max = Value::new(MAX).as_rcrc();
    let imax =
      Value::<isize>::new(max.borrow_mut().output_value() as isize).as_rcrc();
    let one = Value::new(1).as_rcrc();
    let clock = SquareWave::new(one.borrow_mut().output()).as_rcrc();
    let sixteen = Value::new(16).as_rcrc();
    let square = SquareWave::new(sixteen.borrow_mut().output()).as_rcrc();
    let izero = Value::new(0).as_rcrc();
    let select = Select::new(
      square.borrow_mut().output(),
      izero.borrow_mut().output(),
      imax.borrow_mut().output(),
    )
    .as_rcrc();
    let held_value = Feedback::<isize>::new().as_rcrc();
    let never = Value::new(false).as_rcrc();
    let itwo = Value::<isize>::new(2).as_rcrc();
    let div_held_value_by_itwo = Div::<isize>::new(
      held_value.borrow_mut().output(),
      itwo.borrow_mut().output(),
    )
    .as_rcrc();
    let div_new_input_by_itwo = Div::<isize>::new(
      select.borrow_mut().output(),
      itwo.borrow_mut().output(),
    )
    .as_rcrc();
    let add = Add::new(
      div_held_value_by_itwo.borrow_mut().output(),
      div_new_input_by_itwo.borrow_mut().output(),
    )
    .as_rcrc();
    let sample_and_hold = SampleAndHold::new(
      add.borrow_mut().output(),
      clock.borrow_mut().output(),
      never.borrow_mut().output(),
    )
    .as_rcrc();

    held_value
      .borrow_mut()
      .set_input(&sample_and_hold.borrow_mut().output());

    let mut blocks: Vec<RcRcSteppable> = Vec::new();
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &max);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &imax);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &one);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &sixteen);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &izero);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &clock);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &square);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &select);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &held_value);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &div_held_value_by_itwo);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &div_new_input_by_itwo);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &add);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &sample_and_hold);

    for _ in 0..STEPS {
      blocks.iter().for_each(|b| b.borrow_mut().step());
      render(
        b'x',
        b'-',
        add.borrow_mut().output_value() as usize,
        imax.borrow_mut().output_value() as usize,
      );
    }
  }
  Ok(())
}


////////////////////////////////////////////////////////////////////////////////
fn render(
  char: u8,
  char2: u8,
  signal: usize,
  width: usize,
) {
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
