////////////////////////////////////////////////////////////////////////////////////////////////////
fn old_process_file(
  path: &str,
  process_horiz: Box<dyn Fn(&Point, &u8)>,
  process_vert: Box<dyn Fn(&Point, &u8)>,
) -> io::Result<()> {
  let file = File::open(path)?;
  let max_len = max_line_len(path)?;
  let mut buf_reader = BufReader::new(file);
  let mut pos = Point::new(0, 0);
  let mut columns: Vec<Vec<u8>> = Vec::new();

  noisy_println!("max_len:    {}", max_len);

  // loop through the file processing the bytes with process_horiz and building the columns:
  loop {
    let buffer: &[u8] = buf_reader.fill_buf()?;

    if buffer.is_empty() {
      break;
    }

    noisy_println!("-- ls:      {}", columns.format_rows());
    noisy_println!("");

    for &byte in buffer {
      if byte == b'\n' {
        while columns.len() < max_len {
          noisy_println!("Add an imaginary column with one byte and process it!");
          columns.push(vec![b' ']);
          process_horiz(&pos, &b' ');
          pos.col += 1;
        }

        if pos.col < max_len {
          noisy_println!("Lengthen the imaginary column and process the new imaginary byte!");
          while pos.col < max_len {
            columns[pos.col].push(b' ');
            process_horiz(&pos, &b' ');
            pos.col += 1;
          }
        }

        noisy_println!("-- ls:      {}", columns.format_rows());
        noisy_println!("-- c:       {:?}", pos.col);
        noisy_println!("-- l:       {:?}", pos.line);
        noisy_println!("");

        pos.col = 0;
        pos.line += 1;
      } else {
        process_horiz(&pos, &byte);

        if columns.len() > pos.col {
          columns[pos.col].push(byte);
        } else {
          columns.push(vec![byte]);
        }

        pos.col += 1;
      }
    }
    let len = buffer.len();
    buf_reader.consume(len);
  }

  pos.col = 0;
  pos.line = 0;

  noisy_println!("-- ls:  {}", columns.format_rows());
  noisy_println!("");

  // loop through the columns, processing them with the process_vert function:
  columns.each(process_vert);

  Ok(())
}

// let _ = process_file_old(
//   "./data/data.txt",
//   Box::new(|pos: &Point, byte: &u8| {
//     println!("Horiz {}:{}: '{}'", pos.col, pos.line, *byte as char);
//   }),
//   Box::new(|pos: &Point, byte: &u8| {
//     println!("Vert  {}:{}: '{}'", pos.col, pos.line, *byte as char);
//   }),
// );
