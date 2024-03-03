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

    let one = new_rcrc(Value::new(1));
    let max = new_rcrc(Value::new(MAX));
    let clock = new_rcrc(SquareWave::new(one.borrow_mut().output()));
    let ctr_reset = new_rcrc(Feedback::new());
    let ctr_max = new_rcrc(Value::new(MAX));
    let ctr = new_rcrc(UpCounter::new(
      clock.borrow_mut().output(),
      ctr_reset.borrow_mut().output(),
      ctr_max.borrow_mut().output(),
    ));
    let delay_ctr_at_max = new_rcrc(UnitDelay::new(ctr.borrow_mut().at_max()));

    ctr_reset
      .borrow_mut()
      .set_input(&delay_ctr_at_max.borrow_mut().output());

    let mut blocks: Vec<RcRcSteppable> = Vec::new();
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &one);
    // push_onto_vec_of_rcrc_steppable(&mut blocks, &max);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &clock);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_reset);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_max);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr);
    push_onto_vec_of_rcrc_steppable(&mut blocks, &delay_ctr_at_max);

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

    let one = new_rcrc(Value::new(1));
    let max = new_rcrc(Value::new(MAX));
    let clock = new_rcrc(SquareWave::new(one.borrow_mut().output()));
    let ctr_reset = new_rcrc(Feedback::new());
    let ctr_max = new_rcrc(Value::new(MAX));
    let ctr = new_rcrc(UpCounter::new(
      clock.borrow_mut().output(),
      ctr_reset.borrow_mut().output(),
      ctr_max.borrow_mut().output(),
    ));
    let add =
      new_rcrc(Add::new(ctr.borrow_mut().output(), one.borrow_mut().output()));
    let two = new_rcrc(Value::new(2));
    let div =
      new_rcrc(Div::new(add.borrow_mut().output(), two.borrow_mut().output()));
    let square = new_rcrc(SquareWave::new(div.borrow_mut().output()));
    let zero = new_rcrc(Value::new(0));
    let select = new_rcrc(Select::new(
      square.borrow_mut().output(),
      zero.borrow_mut().output(),
      max.borrow_mut().output(),
    ));

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

    let max = new_rcrc(Value::new(MAX));
    let one = new_rcrc(Value::new(1));
    let clock = new_rcrc(SquareWave::new(one.borrow_mut().output()));
    let ctr_reset = new_rcrc(Feedback::new());
    let ctr = new_rcrc(UpCounter::new(
      clock.borrow_mut().output(),
      &ctr_reset.borrow_mut().output(),
      max.borrow_mut().output(),
    ));
    let sr_set = new_rcrc(Feedback::new());
    let sr_reset = new_rcrc(Feedback::new());
    let sr = new_rcrc(SRLatch::new(
      sr_set.borrow_mut().output(),
      sr_reset.borrow_mut().output(),
    ));
    let not_latched = new_rcrc(Not::new(sr.borrow_mut().output()));
    let ctr_at_max_and_latched =
      new_rcrc(And::new(ctr.borrow_mut().at_max(), sr.borrow_mut().output()));
    let ctr_at_max_and_not_latched = new_rcrc(And::new(
      ctr.borrow_mut().at_max(),
      not_latched.borrow_mut().output(),
    ));
    let sub =
      new_rcrc(Sub::new(max.borrow_mut().output(), ctr.borrow_mut().output()));
    let select = new_rcrc(Select::new(
      sr.borrow_mut().output(),
      ctr.borrow_mut().output(),
      sub.borrow_mut().output(),
    ));

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

    let max = new_rcrc(Value::new(MAX));
    let imax =
      new_rcrc(Value::<isize>::new(max.borrow_mut().output_value() as isize));
    let one = new_rcrc(Value::new(1));
    let clock = new_rcrc(SquareWave::new(one.borrow_mut().output()));
    let sixteen = new_rcrc(Value::new(16));
    let square = new_rcrc(SquareWave::new(sixteen.borrow_mut().output()));
    let izero = new_rcrc(Value::new(0));
    let select = new_rcrc(Select::new(
      square.borrow_mut().output(),
      izero.borrow_mut().output(),
      imax.borrow_mut().output(),
    ));
    let held_value = new_rcrc(Feedback::<isize>::new());
    let never = new_rcrc(Value::new(false));
    let itwo = new_rcrc(Value::<isize>::new(2));
    let div_held_value_by_itwo = new_rcrc(Div::<isize>::new(
      held_value.borrow_mut().output(),
      itwo.borrow_mut().output(),
    ));
    let div_new_input_by_itwo = new_rcrc(Div::<isize>::new(
      select.borrow_mut().output(),
      itwo.borrow_mut().output(),
    ));
    let add = new_rcrc(Add::new(
      div_held_value_by_itwo.borrow_mut().output(),
      div_new_input_by_itwo.borrow_mut().output(),
    ));
    let sample_and_hold = new_rcrc(SampleAndHold::new(
      add.borrow_mut().output(),
      clock.borrow_mut().output(),
      never.borrow_mut().output(),
    ));

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
