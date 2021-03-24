use std::mem;

type ChunkIter = std::vec::IntoIter<String>;

pub struct Chunker<I>
where
    I: Iterator<Item = String>,
{
    iter: I,
    curr_chunk: Vec<String>,
    done: bool,
}

impl<I> Chunker<I>
where
    I: Iterator<Item = String>,
{
    pub fn new(iter: I) -> Chunker<I> {
        Chunker {
            iter,
            curr_chunk: Default::default(),
            done: false,
        }
    }
}

impl<I> Iterator for Chunker<I>
where
    I: Iterator<Item = String>,
{
    type Item = ChunkIter;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(line) = self.iter.next() {
                if line.trim().is_empty() {
                    // an empty line will return the current chunk, or if there is nothing in the
                    // current chunk, just continue
                    if self.curr_chunk.is_empty() {
                        continue;
                    }
                    let old_chunk = mem::take(&mut self.curr_chunk);
                    return Some(old_chunk.into_iter());
                }

                self.curr_chunk.push(line.trim().to_string());
            } else {
                if !self.done && !self.curr_chunk.is_empty() {
                    let old_chunk = mem::take(&mut self.curr_chunk);
                    self.done = true;
                    return Some(old_chunk.into_iter());
                }
                return None;
            }
        }
    }
}

pub fn chunks_at_blanks<I>(iter: I) -> Chunker<I>
where
    I: Iterator<Item = String>,
{
    Chunker::new(iter)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_none() {
//         let input = "";
//         assert_eq!(0, chunks_at_blanks(input.lines()).count());
//
//         let input = "\t\t  \t";
//         assert_eq!(0, chunks_at_blanks(input.lines()).count());
//
//         let input = "
//
//         \t\t  \t\t
// ";
//         assert_eq!(0, chunks_at_blanks(input.lines()).count());
//     }
//
//     #[test]
//     fn test_one() {
//         let input = "foo
// bar
// baz";
//         let chunks = chunks_at_blanks(input.lines());
//         assert_eq!(1, chunks.count());
//     }
//
//     #[test]
//     fn test_two() {
//         let input = "one
// two
// three
//
// oone
// ttwo";
//
//         let mut chunks = chunks_at_blanks(input.lines());
//         assert_eq!(3, chunks.next().unwrap().count());
//         assert_eq!(2, chunks.next().unwrap().count());
//         assert!(chunks.next().is_none());
//     }
//
//     #[test]
//     fn test_two_extra_blank_lines() {
//         let input = "one
// two
// three
//
//   \t
//
// oone
// ttwo
// tthree";
//
//         let chunks = chunks_at_blanks(input.lines());
//         assert_eq!(2, chunks.count());
//     }
//
//     #[test]
//     fn test_three() {
//         let input = "one
// two
// three
//
// oone
// ttwo
// tthree
//
// space
// deuce
// trey";
//
//         let chunks = chunks_at_blanks(input.lines());
//         assert_eq!(3, chunks.count());
//     }
// }
