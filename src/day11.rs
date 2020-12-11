use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
enum GridType {
    Floor,
    Empty,
    Occupied,
    Wall,
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    TopLeft,
    TopRight,
    BotLeft,
    BotRight,
}

struct Grid {
    values: Vec<GridType>,
    width: usize,
    version: usize,
}

impl Grid {
    fn new(input: &str, version: usize) -> Self {
        let mut values = Vec::new();
        let mut height = 0;
        for line in input.lines() {
            height += 1;
            for c in line.chars() {
                values.push(match c {
                    'L' => GridType::Empty,
                    '#' => GridType::Occupied,
                    '.' | _ => GridType::Floor,
                });
            }
        }
        let width = values.len() / height;
        Self {
            values: values,
            width: width,
            version: version, // part 1 or part 2
        }
    }
    fn get_at(&self, index: usize) -> GridType {
        if index >= self.values.len() {
            return GridType::Wall;
        } else {
            self.values[index]
        }
    }

    fn get_in_direction(&self, index: usize, dir: Direction) -> (GridType, usize) {
        match dir {
            Direction::Up => match index.checked_sub(self.width) {
                Some(n) => (self.values[n], n),
                None => (GridType::Wall, index),
            },
            Direction::Down => {
                let index = index + self.width;
                (self.get_at(index), index)
            }
            Direction::Left => {
                if index % self.width == 0 {
                    (GridType::Wall, index)
                } else {
                    (self.get_at(index - 1), index - 1)
                }
            }
            Direction::Right => {
                if index % self.width == self.width - 1 {
                    (GridType::Wall, index)
                } else {
                    (self.get_at(index + 1), index + 1)
                }
            }
            Direction::TopLeft => {
                let (_type, i) = self.get_in_direction(index, Direction::Up);
                if _type == GridType::Wall {
                    (GridType::Wall, i)
                } else {
                    self.get_in_direction(i, Direction::Left)
                }
            }
            Direction::TopRight => {
                let (_type, i) = self.get_in_direction(index, Direction::Up);
                if _type == GridType::Wall {
                    (GridType::Wall, i)
                } else {
                    self.get_in_direction(i, Direction::Right)
                }
            }
            Direction::BotLeft => {
                let (_type, i) = self.get_in_direction(index, Direction::Down);
                if _type == GridType::Wall {
                    (GridType::Wall, i)
                } else {
                    self.get_in_direction(i, Direction::Left)
                }
            }
            Direction::BotRight => {
                let (_type, i) = self.get_in_direction(index, Direction::Down);
                if _type == GridType::Wall {
                    (GridType::Wall, i)
                } else {
                    self.get_in_direction(i, Direction::Right)
                }
            }
        }
    }

    fn get_occ_neighbours_part2(&self, index: usize, stop_on_floor: bool) -> usize {
        let mut num_occ = 0;
        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::TopLeft,
            Direction::TopRight,
            Direction::BotLeft,
            Direction::BotRight,
        ] {
            let mut index = index;
            loop {
                let (_type, i) = self.get_in_direction(index, *dir);
                match _type {
                    GridType::Floor => {
                        if stop_on_floor {
                            break;
                        } else {
                            index = i
                        }
                    }
                    GridType::Occupied => {
                        num_occ += 1;
                        break;
                    }
                    GridType::Empty | GridType::Wall => {
                        break;
                    }
                }
            }
        }
        num_occ
    }

    fn update(&mut self) -> (bool, usize) {
        let mut new_values: Vec<GridType> = self.values.clone();
        let mut changes = 0;
        let mut num_occ_total = 0;
        for i in 0..self.values.len() {
            let num_occ_neighbours = if self.version == 1 {
                self.get_occ_neighbours_part2(i, true)
            } else {
                self.get_occ_neighbours_part2(i, false)
            };
            let thresh_occ = if self.version == 1 { 4 } else { 5 };
            if self.values[i] == GridType::Empty && num_occ_neighbours == 0 {
                new_values[i] = GridType::Occupied;
                changes += 1;
            } else if self.values[i] == GridType::Occupied && num_occ_neighbours >= thresh_occ {
                new_values[i] = GridType::Empty;
                changes += 1;
            }

            if new_values[i] == GridType::Occupied {
                num_occ_total += 1;
            }
        }
        self.values = new_values;
        (changes != 0, num_occ_total)
    }
}

fn iterate_until_stable(seats: &mut Grid) -> usize {
    loop {
        let (changed, num_occ) = seats.update();
        if !changed {
            return num_occ;
        }
    }
}

pub fn main() {
    let input = read_to_string("inputs/day11.txt").unwrap();

    // PART 1
    let start = Instant::now();
    let known_answer = "2247";
    let part_1: usize = iterate_until_stable(&mut Grid::new(&input, 1));
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "2011";
    let part_2: usize = iterate_until_stable(&mut Grid::new(&input, 2));
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        let answer: usize = iterate_until_stable(&mut Grid::new(&input, 1));
        assert_eq!(answer, 37);
    }

    #[test]
    fn test_example_2() {
        let input: &str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        let answer: usize = iterate_until_stable(&mut Grid::new(&input, 2));
        assert_eq!(answer, 26);
    }
}
