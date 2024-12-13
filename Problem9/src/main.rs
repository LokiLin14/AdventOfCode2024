mod prelude;

use std::cmp::{max, min};
use prelude::*;
use std::collections::BTreeSet;
use std::path::Path;

fn main() {
    solve_part1(true);
    solve_part1(false);
    solve_part2(true);
    solve_part2(false);
}

fn arithmetic_sum(start : i64, length : i64) -> i64 {
    length * (start + start + (length - 1)) / 2
}
fn solve_part2(solve_example: bool) {
    println!("Solving part2 with file \"part1-{}\"", if solve_example { "example" } else { "test" });

    let problem = get_input(1, solve_example);
    let disk = FileCompactedDisk::from_str(&problem);

    let answer = disk.segments.iter().map(|f| {
        let len = f.length as i64;
        let start_pos = f.start_pos as i64;
        f.file_index as i64 * arithmetic_sum(start_pos, len)
    }).sum::<i64>();

    println!("Answer: {answer}");
}

#[derive(Debug, Clone, Copy)]
struct FileSegment {
    start_pos: usize,
    file_index: usize,
    length: usize,
}
#[derive(Debug, Clone)]
struct FileCompactedDisk {
    length : usize,
    segments: Vec<FileSegment>,
}

#[derive(Debug, Clone)]
struct FileCompactedDiskIterator<'a> {
    disk : &'a FileCompactedDisk,
    segments_idx : usize,
    index_in_disk: usize,
}

impl Iterator for FileCompactedDiskIterator<'_> {
    // first is file_index, second is position in the disk
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_in_disk >= self.disk.length {
            return None;
        }
        let ret : i32;
        let file : FileSegment = self.disk.segments[self.segments_idx];
        if self.index_in_disk - file.start_pos < file.length {
            ret = file.file_index as i32;
        } else {
            ret = -1;
        }
        self.index_in_disk += 1;
        if self.segments_idx + 1 < self.disk.segments.len()
            && self.disk.segments[self.segments_idx + 1].start_pos <= self.index_in_disk {
            self.segments_idx += 1;
        }
        Some(ret)
    }
}

impl FileCompactedDisk {
    fn file_pages(&self) -> FileCompactedDiskIterator {
        FileCompactedDiskIterator {
            disk: self,
            segments_idx: 0,
            index_in_disk: 0
        }
    }

    fn from_str(str: &str) -> FileCompactedDisk {
        // preprocess data to file segments and empty segments so we can update it quickly
        let mut head: usize = 0;
        let mut str_head: usize = 0;
        // maps (size of segment, position of segment)
        let mut empty_segments : Vec<BTreeSet<usize>> = vec![BTreeSet::new(); 10];
        let mut file_segments = Vec::<FileSegment>::new();
        let mut length = 0;
        str.chars().enumerate().for_each(|(i, c)| {
            let len = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                // this segment is a file segment
                file_segments.push(FileSegment {
                    start_pos: head,
                    file_index: i / 2,
                    length: len
                });
            } else {
                // this segment is an empty segment
                empty_segments[len].insert(head);
            }
            length = head + len;
            head += len;
        });

        // update the file segments
        file_segments.iter_mut().rev().for_each(|f| {
            let mut best = None;
            for i in (f.length..=9) {
                if let Some(&f) = empty_segments[i].first() {
                    let cur = (f, i);
                    best = Some(if let Some(b) = best {
                        min(b, cur)
                    } else {
                        cur
                    })
                }
            }
            let emp;
            if let Some(b) = best {
                emp = b;
            } else {
                return;
            }

            // make sure we are not moving files backwards
            if emp.0 > f.start_pos {
                return;
            }

            // move the file forwards
            f.start_pos = emp.0;

            // update empty_segments to reflect the change
            empty_segments[emp.1].remove(&emp.0);
            let remaining_length = emp.1 - f.length;
            let new_starting_pos = emp.0 + f.length;
            if remaining_length > 0 {
                empty_segments[remaining_length].insert(new_starting_pos);
            }
        });
        file_segments.sort_by(|a, b| a.start_pos.cmp(&b.start_pos));

        FileCompactedDisk {
            length,
            segments: file_segments
        }
    }
}

fn solve_part1(solve_example: bool) {
    let problem = get_input(1, solve_example);
    println!("Solving part1-{}", if solve_example { "example" } else { "test" });
    // println!("Disk is: {problem:?}");
    // let expanded_representation = expand_string(&problem);
    // println!("Disk expanded to: {expanded_representation:?}");
    // let compacted = CompactedDiskIterator::from_string(&problem).collect::<Vec<_>>();
    // println!("Compacted to: {compacted:?}");
    let answer = PageCompactedDisk::from_string(&problem)
        .enumerate()
        .map(|(i, s)| i as i64 * s as i64).sum::<i64>();
    println!("Answer: {answer:?}");
}

#[derive(Debug, Clone)]
struct PageCompactedDisk {
    expanded_representation: Vec<i32>,
    start: usize,
    end: usize,
}

impl PageCompactedDisk {
    fn from_string(s: &str) -> PageCompactedDisk {
        let expanded_representation = expand_string(&s);

        PageCompactedDisk {
            end: expanded_representation.len(),
            start: 0,
            expanded_representation,
        }
    }

    fn get_page_from_back(&mut self) -> Option<i32> {
        while self.start < self.end && self.expanded_representation[self.end - 1] == -1 {
            self.end -= 1;
        }
        if self.start == self.end {
            None
        } else {
            let ret = Some(self.expanded_representation[self.end - 1]);
            self.end -= 1;
            ret
        }
    }
}

impl Iterator for PageCompactedDisk {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        if self.expanded_representation[self.start] == -1 {
            self.start += 1;
            self.get_page_from_back()
        } else {
            let ret = self.expanded_representation[self.start];
            self.start += 1;
            Some(ret)
        }
    }
}
fn expand_string(disk_string: &str) -> Vec<i32> {
    let inf_seq = 0..;
    disk_string.chars().zip(inf_seq).flat_map(|(c, idx)| {
        let len = c.to_digit(10).unwrap();
        return if idx % 2 == 0 {
            let digit = idx / 2;
            vec![digit; len as usize]
        } else {
            vec![-1; len as usize]
        };
    }).collect::<Vec<i32>>()
}