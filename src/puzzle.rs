#![allow(dead_code)]

use crate::solver::Spot;

pub struct Puzzle{
    data: Vec<Vec<i32>>,
    is_solved: bool,
    spots: Vec<Spot>
}

impl Puzzle {
    pub fn from_array(puzzle: Vec<Vec<i32>>) -> Self {
        Puzzle {
            data: puzzle.clone(),
            is_solved: false,
            spots: vec![]
        }
    }

    pub fn from_spots(spots: Vec<Spot>) -> Self {
        let mut data: Vec<Vec<i32>> = Vec::new();
        let mut spots_copy = spots.clone();
        spots_copy.sort_by(|x, y| x.column_index.cmp( &y.column_index));
        let last_column = spots_copy.last().unwrap().column_index + 1;
        let (mut left, mut right) = spots.split_at(last_column as usize);
        data.push(left.iter().map(|x| x.number).collect::<Vec<_>>());
        while right.len() >= last_column as usize {
            (left, right) = right.split_at(last_column as usize);
            data.push(left.iter().map(|x| x.number).collect::<Vec<_>>());
        }
        Puzzle {
            data,
            is_solved: false,
            spots: spots.clone()
        }
    }
    
    pub fn to_string(&self) -> String {
        let mut str= String::new();
        for row in self.data.clone() {
            let row_str = row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
            str.push_str(row_str.as_str());
            str.push_str("\n");
        }
        str
    }
}