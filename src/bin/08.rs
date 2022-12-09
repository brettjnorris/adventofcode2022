use itertools::Itertools;

enum SliceType {
    Row,
    Column,
}

pub struct Matrix {
    elems: Vec<u32>,
    num_rows: u32,
    num_columns: u32,
}

impl Matrix {
    fn from_input(input: &str) -> Matrix {
        let lines = input.lines();
        let num_rows: u32 = lines.clone().count() as u32;
        let num_columns: u32 = lines.collect::<Vec<&str>>().get(0).unwrap().len() as u32;

        let elems = Matrix::concat_input_lines(input);

        Matrix {
            elems,
            num_rows,
            num_columns,
        }
    }

    fn concat_input_lines(input: &str) -> Vec<u32> {
        input
            .lines()
            .map(|line| {
                line.split("")
                    .filter_map(|val| match val.parse::<u32>() {
                        Ok(val) => Some(val),
                        _ => None,
                    })
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect::<Vec<u32>>()
    }

    fn max_distance(&self) -> u32 {
        self.elems
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, elem)| self.distance_by_elem(elem, &i))
            .max()
            .unwrap()
    }

    fn visible_elems(&self) -> u32 {
        self.elems
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(i, _elem)| self.is_elem_visible(&i))
            .count() as u32
    }

    fn is_elem_visible(&self, index: &usize) -> bool {
        let col_index = index % self.num_columns as usize;
        let row_index = index / self.num_rows as usize;

        let column = self.slice(SliceType::Column, col_index);
        let row = self.slice(SliceType::Row, row_index);

        let mut visible = vec![];
        visible.push(self.visible_by_slice(&column, SliceType::Column, col_index));
        visible.push(self.visible_by_slice(&row, SliceType::Row, row_index));

        let flattened = visible.into_iter().flatten().collect::<Vec<u32>>();

        flattened.iter().any(|n| *n == *index as u32)
    }

    fn distance_by_elem(&self, elem: u32, index: &usize) -> u32 {
        let col_index = index % self.num_columns as usize;
        let row_index = index / self.num_rows as usize;

        let column = self.slice(SliceType::Column, col_index);
        let row = self.slice(SliceType::Row, row_index);

        let column_distance = Matrix::distance_by_slice(&column, row_index, elem);
        let row_distance = Matrix::distance_by_slice(&row, col_index, elem);

        column_distance * row_distance
    }

    fn distance_by_slice(slice: &Vec<u32>, index: usize, elem: u32) -> u32 {
        let (left, right) = slice.split_at(index);

        let mut left_distance: u32 = 0;
        let mut right_distance: u32 = 0;

        for n in left.into_iter().rev() {
            left_distance = left_distance + 1;
            if n >= &elem {
                break;
            }
        }

        for n in right[1..].into_iter() {
            right_distance = right_distance + 1;
            if n >= &elem {
                break;
            }
        }

        left_distance * right_distance
    }

    fn visible_by_slice(
        &self,
        slice: &Vec<u32>,
        slice_type: SliceType,
        slice_index: usize,
    ) -> Vec<u32> {
        let mut visible_elems: Vec<u32> = vec![];

        let mut max: i32 = -1;
        for (i, elem) in slice.iter().enumerate() {
            if *elem as i32 > max {
                let elem_index = match slice_type {
                    SliceType::Row => (slice_index as u32 * self.num_columns) + i as u32,
                    SliceType::Column => (i as u32 * self.num_rows) + slice_index as u32,
                };

                visible_elems.push(elem_index as u32);
                max = *elem as i32;
            }
        }

        max = -1;
        for (i, elem) in slice.iter().rev().enumerate() {
            if *elem as i32 > max {
                let rev_index = ((slice.len() - 1) - i) as u32;
                let elem_index = match slice_type {
                    SliceType::Row => (slice_index as u32 * self.num_columns) + rev_index as u32,
                    SliceType::Column => (rev_index as u32 * self.num_rows) + slice_index as u32,
                };
                visible_elems.push(elem_index as u32);
                max = *elem as i32;
            }
        }

        visible_elems.into_iter().unique().collect::<Vec<u32>>()
    }

    fn slice(&self, slice_type: SliceType, index: usize) -> Vec<u32> {
        self.elems
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(i, elem)| match slice_type {
                SliceType::Row => match i / (self.num_columns as usize) == index {
                    true => Some(elem),
                    false => None,
                },
                SliceType::Column => match i % (self.num_columns as usize) == index {
                    true => Some(elem),
                    false => None,
                },
            })
            .collect::<Vec<u32>>()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = Matrix::from_input(input);
    Some(matrix.visible_elems())
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = Matrix::from_input(input);
    Some(matrix.max_distance())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }

    #[test]
    fn test_matrix_col() {
        let matrix = Matrix {
            elems: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
            num_columns: 3,
            num_rows: 3,
        };

        assert_eq!(matrix.slice(SliceType::Column, 0), vec![0, 3, 6]);
        assert_eq!(matrix.slice(SliceType::Column, 1), vec![1, 4, 7]);
        assert_eq!(matrix.slice(SliceType::Column, 2), vec![2, 5, 8]);
    }

    #[test]
    fn test_matrix_row() {
        let matrix = Matrix {
            elems: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
            num_columns: 3,
            num_rows: 3,
        };

        assert_eq!(matrix.slice(SliceType::Row, 0), vec![0, 1, 2]);
        assert_eq!(matrix.slice(SliceType::Row, 1), vec![3, 4, 5]);
        assert_eq!(matrix.slice(SliceType::Row, 2), vec![6, 7, 8]);
    }

    #[test]
    fn test_concat_input_lines() {
        let input = "30373\n25512";
        assert_eq!(
            Matrix::concat_input_lines(input),
            vec![3, 0, 3, 7, 3, 2, 5, 5, 1, 2]
        )
    }

    #[test]
    fn test_matrix_visible_by_slice() {
        let matrix = Matrix {
            elems: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
            num_columns: 3,
            num_rows: 3,
        };

        let slice: Vec<u32> = matrix.slice(SliceType::Column, 0);

        assert_eq!(
            matrix.visible_by_slice(&slice, SliceType::Column, 0),
            vec![0, 3, 6]
        );
    }

    #[test]
    fn test_distance_by_elem() {
        let matrix = Matrix {
            elems: vec![
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ],
            num_columns: 5,
            num_rows: 5,
        };

        assert_eq!(matrix.distance_by_elem(5, &17), 8);
    }
}
