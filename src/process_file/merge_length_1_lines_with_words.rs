use crate::simple_geo::ConnectedLine;
use crate::simple_geo::LineMethods;
use crate::simple_geo::Offsetable;
use crate::simple_geo::Orientation::Horizontal;
use crate::simple_geo::Word;
use crate::util::vec_utils::Removeql;
use crate::util::vec_utils::SortedInsert;

////////////////////////////////////////////////////////////////////////////////
pub fn merge_length_1_lines_with_words(
  mut lines: Vec<ConnectedLine>,
  mut words: Vec<Word>,
) -> (Vec<ConnectedLine>, Vec<Word>) {
  let single_length_lines = lines
    .iter()
    .filter(|cl| cl.len() == 1 && cl.orientation == Horizontal)
    .cloned()
    .collect::<Vec<ConnectedLine>>();

  for line in single_length_lines {
    println!("Length 1 line: {:?}", line);

    let candidate_words = words
      .iter()
      .filter(|word| word.start == line.start.offset_by(0, 1))
      .cloned()
      .collect::<Vec<Word>>();

    if candidate_words.len() == 0 {
      panic!("No match for {:?} found.", line);
    }
    else if candidate_words.len() > 1 {
      panic!("Bad data, found more than one candidate word for {:?}", line);
    }
    else {
      println!("  Candidate word: {:?}", candidate_words[0]);

      let new_word = Word::new(
        &format!("-{}", candidate_words[0].string),
        line.start,
        candidate_words[0].end,
      )
      .unwrap();

      println!("  New word: {:?}", new_word);

      lines.removeql(&line);
      words.removeql(&candidate_words[0]);
      words.sorted_insert(new_word);
    }
  }

  (lines, words)
}
