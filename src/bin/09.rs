use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

#[derive(Clone)]
struct Chunk {
    content: Vec<usize>,
    after: u32,
}

impl Chunk {
    fn pop_block(&mut self) -> Result<usize> {
        let popped = self.content.pop();
        match popped {
            Some(popped) => {
                self.after += 1;
                Ok(popped)
            }
            None => Err(Error::msg("No content to pop")),
        }
    }

    fn push_block(&mut self, block: usize) -> Result<()> {
        if self.after == 0 {
            return Err(Error::msg("Cannot push to full block"));
        }
        self.content.push(block);
        self.after -= 1;
        Ok(())
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.content.iter().join(""),
            (0..self.after).map(|_| '.').join("")
        )
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> Vec<Chunk> {
        let contents = reader.lines().flatten().join("");
        let chunk_pairs = contents
            .chars()
            .map(|char| char.to_digit(10).unwrap())
            .chunks(2);
        chunk_pairs
            .into_iter()
            .enumerate()
            .map(|(i, mut chunk)| {
                let file_size = chunk.next().unwrap();
                let after_option = chunk.next().unwrap_or(0);
                Chunk {
                    content: Vec::from_iter((0..file_size).map(|_| i)),
                    after: after_option,
                }
            })
            .collect_vec()
    }

    fn debug_chunks(chunks: &Vec<Chunk>) {
        // print!("\x1B[2J\x1B[1;1H");
        println!("{}", chunks.into_iter().join(""));
        // println!(
        //     "{}\n",
        //     chunks
        //         .into_iter()
        //         .map(|chunk| {
        //             if chunk.after == 0 {
        //                 '#'
        //             } else if chunk.content.len() == 0 {
        //                 '.'
        //             } else {
        //                 'O'
        //             }
        //         })
        //         .join("")
        // )
    }

    fn checksum(chunks: Vec<Chunk>) -> usize {
        let zero_arr: [usize; 1] = [0];
        let blocks = chunks.iter().flat_map(|chunk| {
            let content = chunk.content.iter();
            let after = zero_arr
                .iter()
                .cycle()
                .take(chunk.after.try_into().unwrap());
            content.chain(after).collect_vec()
        });

        blocks
            .enumerate()
            .fold(0, |acc, (position, id)| acc + position * *id)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut chunks = parse(reader);

        fn chunk_has_content(chunk: &Chunk) -> bool {
            chunk.content.len() != 0
        }

        fn chunk_has_space(chunk: &Chunk) -> bool {
            chunk.after > 0
        }

        fn more_to_process(chunks: &Vec<Chunk>) -> bool {
            let (pop_idx, _) = chunks
                .iter()
                .rev()
                .find_position(|chunk| chunk_has_content(chunk))
                .unwrap();
            let pop_idx = chunks.len() - pop_idx - 1;
            let (push_idx, _) = chunks
                .iter()
                .find_position(|chunk| chunk_has_space(chunk))
                .unwrap();
            pop_idx > push_idx
        }

        let mut last_popped_chunk_position: usize = 0;
        let mut last_pushed_chunk_position: usize = 0;

        // debug_chunks(&chunks);
        while more_to_process(&chunks) {
            let (pop_offset, chunk_to_pop) = chunks
                .iter_mut()
                .rev()
                .skip(last_popped_chunk_position)
                .find_position(|chunk| chunk_has_content(chunk))
                .unwrap();
            last_popped_chunk_position += pop_offset;

            let block = chunk_to_pop.pop_block().unwrap();

            let (push_offset, chunk_to_push) = chunks
                .iter_mut()
                .skip(last_pushed_chunk_position)
                .find_position(|chunk| chunk_has_space(chunk))
                .unwrap();
            last_pushed_chunk_position += push_offset;

            chunk_to_push.push_block(block).unwrap();
            // debug_chunks(&chunks);
        }

        Ok(checksum(chunks))
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut chunks = parse(reader);

        fn find_chunk_span_fits(chunks: &Vec<Chunk>, file_to_move: &Chunk) -> Option<usize> {
            chunks
                .iter()
                .find_position(|chunk| {
                    chunk.after >= file_to_move.content.len().try_into().unwrap()
                })
                .map(|(idx, _)| idx)
        }

        let mut idx = chunks.len() - 1;
        loop {
            // debug_chunks(&chunks);
            let file_to_move = &chunks[idx];
            let fitting_span = find_chunk_span_fits(&chunks, file_to_move);
            let content_size: u32 = file_to_move.content.len().try_into().unwrap();
            let fitting_idx = match fitting_span {
                Some(i) => {
                    if i >= idx {
                        // println!(
                        //     "No span left of '{}' fits {} blocks. Continuing.",
                        //     file_to_move.content[0], content_size
                        // );
                        if idx == 0 {
                            break;
                        } else {
                            idx -= 1;
                            continue;
                        }
                    } else {
                        i
                    }
                }
                None => {
                    // println!("No span fits {} blocks. Continuing.", content_size);

                    if idx == 0 {
                        break;
                    } else {
                        idx -= 1;
                        continue;
                    }
                }
            };
            let content = file_to_move.content.clone();
            let fitting_chunk = chunks.get_mut(fitting_idx).unwrap();
            let remaining_after = fitting_chunk.after - content_size;
            fitting_chunk.after = 0;

            chunks.insert(
                fitting_idx + 1,
                Chunk {
                    content,
                    after: remaining_after,
                },
            );

            let file_to_move = &mut chunks[idx + 1];
            file_to_move.content.clear();
            file_to_move.after += content_size;
        }
        // debug_chunks(&chunks);

        Ok(checksum(chunks))
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
