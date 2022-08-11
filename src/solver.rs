use crate::Puzzle;
use crate::utils::index_of;

#[derive(Debug, Clone)]
pub struct Spot{
    pub(crate) row_index: i32,
    pub(crate) column_index: i32,
    entropy: i32,
    pub(crate) number: i32,
    clusters: Vec<Vec<Spot>>
}

impl Spot {
    pub fn new(number: i32, row: i32, column: i32, entropy: i32) -> Self {
        Spot {
            row_index: row,
            column_index: column,
            entropy,
            number,
            clusters: Vec::new()
        }
    }
    
    pub fn set_clusters(&mut self, clusters: Vec<Vec<Spot>>) {
        self.clusters = clusters;
    }
    
    pub fn update_entropy(&mut self) {
        // If the spot has a number there is no need to recalculate the entropy
        // if self.number != 0{
        //     self.entropy = 0;
        //     return;
        // }
        let mut possibilities = (1..self.entropy + 1).collect::<Vec<i32>>();
        for (_index, cluster) in self.clusters.iter().enumerate() {
            for spot in cluster {
                if possibilities.contains(&spot.number){
                    // If the spot is present in the possibilities remove it.
                    possibilities.remove(index_of(possibilities.clone(), spot.number).unwrap());
                }
            }
        }
        
        
        if possibilities.len() == 1 {
            self.number = *possibilities.get(0).unwrap();
            self.entropy = 0;
        } else {
            // TODO: Make a random choice to throw a dice.
            self.entropy = possibilities.len() as i32;
        }
    }
}

pub fn solve_puzzle(puzzle_input: Vec<Vec<i32>>, grid_dimension: (i32, i32)) -> Puzzle{
    // Create spots for the numbers
    let mut spots: Vec<Spot> = Vec::new();
    for (row_index, row) in puzzle_input.iter().enumerate(){
        for (column_index, number) in row.iter().enumerate() {
            spots.push(Spot::new(*number as i32,
                                 row_index as i32,
                                 column_index as i32,
                                 if number == &0i32 { puzzle_input.len() as i32 } else { 0i32 }));
        }
    }
    let mut spots_clone = spots.clone();
    for (_index, spot) in spots_clone.iter_mut().enumerate() {
        let mut row_cluster: Vec<Spot> = Vec::new();
        let mut column_cluster: Vec<Spot> = Vec::new();
        let mut grid_cluster: Vec<Spot> = Vec::new();
        for second_spot in spots.clone() {
            if spot.row_index == second_spot.row_index as i32 {
                row_cluster.push(second_spot.clone());
            }
            if spot.column_index == second_spot.column_index {
                column_cluster.push(second_spot.clone());
            }
            let row_iters = puzzle_input.len() as i32 / grid_dimension.0;
            for row_iter in [..row_iters] {
                let min_grid_index = row_iter.end * grid_dimension.0;
                let max_grid_index = min_grid_index + grid_dimension.0;
                if second_spot.row_index >= min_grid_index && second_spot.row_index < max_grid_index{
                    grid_cluster.push(second_spot.clone());
                }
            }
            let col_iters = puzzle_input.get(0).unwrap().len() as i32 / grid_dimension.1;
            for col_iter in [..col_iters] {
                let min_grid_index = col_iter.end * grid_dimension.1;
                let max_grid_index = min_grid_index + grid_dimension.1;
                if second_spot.column_index >= min_grid_index && second_spot.column_index < max_grid_index{
                    grid_cluster.push(second_spot.clone());
                }
            }
        }
        println!("{:#?}", row_cluster.clone());
        spot.set_clusters(vec![row_cluster, column_cluster, grid_cluster]);
    }
    
    let mut solved_spots = Vec::new();
    for mut spot in spots_clone{
        spot.update_entropy();
        solved_spots.push(spot);
    }
    // spots_clone.split
    Puzzle::from_spots(solved_spots)
}
