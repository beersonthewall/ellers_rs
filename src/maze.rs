//! # Ellers_rs
//! An implementation of Eller's maze generation algorithm.
//!
//! ## Algorithm
//! ### Initialization
//! 1) Create empty row
//! 2) Add cells to their own unique sets
//! 3) From left to right randomly add left/right walls
//!    If we choose not to add a wall, union the sets to which the current cell and
//!    cell to the right are members
//! 4) Create bottom walls moving left to right randomly choose to add a wall
//!    Each set must have at least one cell without a bottom wall
//!
//! ### Generating the next row
//! 1) Copy Previous row to next_row
//! 2) remove right walls.
//! 3) if cell.walls.contains(Wall::Bottom) set_id = 0;
//! 4) remove bottom walls
//! 5) cells without a set get their own unique set
//! 6) randomly add right walls, merging sets when not adding a wall
//!    If two adjacent cells are in the same set, we must add a wall
//! 7) randomly add bottom walls, each set must have at least one cell without a bottom wall
//!
//! ### Completing the maze
//! 1) create a normal row, except each cell has a bottom wall
//! 2) remove walls between cells that are members of different sets
//!    union sets until all cells are members of the same set.

use rand::random;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum Wall {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
struct Cell {
    walls: HashSet<Wall>,
    label: usize,
    set_id: usize,
}

impl Cell {
    fn new(label: usize, set_id: usize) -> Self {
        Cell {
            walls: HashSet::new(),
            label: label,
            set_id: set_id,
        }
    }
}

#[derive(Debug)]
pub struct MazeBuilder {
    sets: HashMap<usize, HashSet<usize>>,
    cells: HashMap<usize, Cell>,
    width: usize,
    set_cnt: usize,
    label_cnt: usize,
    row: Vec<usize>,
    iterations: usize,
}

impl MazeBuilder {

    /// Upsets the set 'to' with the union of 'to' and 'from'. Removes 'from'.
    /// Updates all cell set_id fields.
    fn merge_sets(&mut self, from: usize, to: usize) {
        let sets = &mut self.sets;
        let cells = &mut self.cells;

        // Update all set_ids on cells in from set.
        if let Some(from_set) = sets.get(&from) {
            from_set.iter().for_each(|label| {
                if let Some(cell) = cells.get_mut(label) {
                    cell.set_id = to;
                }
            });
        }

        if let Some(from_set) = sets.remove(&from) {
            if let Some(to_set) = sets.get(&to) {
                let union: HashSet<usize> = from_set.union(&to_set).cloned().collect();
                sets.insert(to, union);
            }
        }
    }

    /// Generates the next row in the maze and drops the previous row. Returns list of cell labels.
    pub fn ellers(&mut self) -> &Vec<usize> {
        let row = &mut self.row;
        let mut new_row = row.clone();

        for i in 0..new_row.len() {
            // Clone cell "above" new_row cell.
            new_row[i] = self.label_cnt;
            let mut new_cell = self.cells.get(&row[i]).unwrap().clone();
            new_cell.label = self.label_cnt;

            let set = self.sets.entry(new_cell.set_id).or_insert(HashSet::new());
            set.insert(self.label_cnt);
            self.cells.insert(new_cell.label, new_cell);

            if let Some(cell) = self.cells.get_mut(&self.label_cnt) {
                cell.walls.remove(&Wall::Top);
                cell.walls.remove(&Wall::Right);
                cell.walls.remove(&Wall::Left);

                if cell.walls.remove(&Wall::Bottom) {
                    cell.walls.insert(Wall::Top);

                    let old_set = self.sets.entry(cell.set_id).or_insert(HashSet::new());
                    old_set.remove(&cell.label);

                    cell.set_id = self.set_cnt;
                    self.set_cnt += 1;

                    let set = self.sets.entry(cell.set_id).or_insert(HashSet::new());
                    set.insert(cell.label);
                }
            }

            self.label_cnt += 1;
        }

        let mut iter = new_row.iter().peekable();
        while let Some(i) = iter.next() {
            let cells = &mut self.cells;

            let mut merge: bool = false;
            let mut add_left: bool = false;
            let mut next_set = 0;

            // Get next set
            if let Some(next_label) = iter.peek() {
                if let Some(next_cell) = cells.get_mut(next_label) {
                    next_set = next_cell.set_id;
                }
            }

            if let Some(cell) = cells.get_mut(&i) {
                if next_set != 0 && next_set == cell.set_id {
                    cell.walls.insert(Wall::Right);
                    add_left = true;
                } else if random() {
                    cell.walls.insert(Wall::Right);
                    add_left = true;
                } else {
                    merge = true;
                }
            }

            if let Some(next_label) = iter.peek() {
                let current_set_id = match cells.get(&i) {
                    Some(cell) => cell.set_id,
                    None => 0,
                };

                if add_left {
                    if let Some(cell) = cells.get_mut(&next_label) {
                        cell.walls.insert(Wall::Left);
                    }
                }

                // Use flags to avoid two mutable references.
                if merge && current_set_id != 0 {
                    let from: usize = cells.get(&next_label).unwrap().set_id;
                    self.merge_sets(from, current_set_id);
                }
            }
        }

        // Make sure outside edges have walls.
        if let Some(cell) = self.cells.get_mut(&new_row[0]) {
            cell.walls.insert(Wall::Left);
        }
        if let Some(cell) = self.cells.get_mut(&new_row[new_row.len() - 1]) {
            cell.walls.insert(Wall::Right);
        }

        // Remove cells and their labels from sets before the row they are in is dropped.
        for i in &self.row {
            let mut set_id = 0;
            if let Some(cell) = self.cells.remove(i) {
                set_id = cell.set_id;
            }

            if set_id != 0 {
                if let Some(set) = self.sets.get_mut(&set_id) {
                    set.remove(i);
                }
            }
        }

        self.row = new_row;
        self.generate_bottom_walls();
        &self.row
    }

    /// Generates the ending row which obeys different rules.
    pub fn end(&mut self) {
        self.ellers();
        for i in &self.row {
            if let Some(cell) = self.cells.get_mut(i) {
                cell.walls.insert(Wall::Bottom);
            }
        }

        let mut iter = self.row.iter().peekable();
        while let Some(label) = iter.next() {
            let mut set_id = 0;
            if let Some(next_label) = iter.peek() {
                if let Some(next_cell) = self.cells.get(next_label) {
                    set_id = next_cell.set_id;
                }
            }

            let mut union = true;
            let mut target_set = 0;
            if set_id != 0 {
                if let Some(cell) = self.cells.get_mut(label) {
                    if set_id != cell.set_id {
                        union = true;
                        target_set = cell.set_id;
                        cell.walls.remove(&Wall::Right);
                    }
                }
            }

            if union {
                if let Some(next_label) = iter.peek() {
                    if let Some(next_cell) = self.cells.get_mut(next_label) {
                        next_cell.walls.remove(&Wall::Left);
                    }
                }

                let mut source = Vec::new();
                if let Some(source_set) = self.sets.get(&set_id) {
                    for i in source_set {
                        source.push(*i);
                    }
                }
                if let Some(target_set) = self.sets.get_mut(&target_set) {
                    for i in source {
                        target_set.insert(i);
                    }
                }
            }
        }
    }

    pub fn new(width: usize, iterations: usize) -> MazeBuilder {
        let mut maze_bldr = MazeBuilder {
            sets: HashMap::new(),
            cells: HashMap::new(),
            width: width,
            set_cnt: 1,
            label_cnt: 0,
            row: Vec::new(),
            iterations: iterations,
        };

        // Generate the initial row and put each cell into it's own set.
        while maze_bldr.label_cnt < maze_bldr.width {
            maze_bldr.cells.insert(
                maze_bldr.label_cnt,
                Cell::new(maze_bldr.label_cnt, maze_bldr.set_cnt),
            );

            maze_bldr.row.push(maze_bldr.label_cnt);
            maze_bldr
                .cells
                .get_mut(&maze_bldr.label_cnt)
                .unwrap()
                .walls
                .insert(Wall::Top);
            let set = maze_bldr
                .sets
                .entry(maze_bldr.set_cnt)
                .or_insert(HashSet::new());
            set.insert(maze_bldr.label_cnt);

            maze_bldr.label_cnt += 1;
            maze_bldr.set_cnt += 1;
        }

        maze_bldr
            .cells
            .get_mut(&0)
            .unwrap()
            .walls
            .insert(Wall::Left);
        maze_bldr
            .cells
            .get_mut(&(width - 1))
            .unwrap()
            .walls
            .insert(Wall::Right);

        maze_bldr.init_vertical_walls();
        maze_bldr.generate_bottom_walls();

        maze_bldr
    }

    fn generate_bottom_walls(&mut self) {
        for x in 1..self.width - 1 {
            if random() {
                let label = self.row[x];
                if let Some(cell) = self.cells.get_mut(&label) {
                    cell.walls.insert(Wall::Bottom);
                }
            }
        }
        for x in 1..self.width - 1 {
            let label = self.row[x];
            let set_label = self.cells.get(&label).unwrap().set_id;
            if let Some(set) = self.sets.get(&set_label) {
                let mut has_down_passage = false;
                for cell_label in set {
                    if !self
                        .cells
                        .get(&cell_label)
                        .unwrap()
                        .walls
                        .contains(&Wall::Bottom)
                    {
                        has_down_passage = true;
                        break;
                    }
                }
                if !has_down_passage {
                    if let Some(cell) = self.cells.get_mut(&label) {
                        cell.walls.remove(&Wall::Bottom);
                    }
                }
            }
        }
    }

    fn init_vertical_walls(&mut self) {
        for x in 0..self.width - 1 {
            if random() {
                let current_label = self.row[x];
                let next_label = self.row[x + 1];
                if let Some(cell) = self.cells.get_mut(&current_label) {
                    cell.walls.insert(Wall::Right);
                }
                if let Some(cell) = self.cells.get_mut(&next_label) {
                    cell.walls.insert(Wall::Left);
                }
            } else {
                let l1 = self.row[x];
                let l2 = self.row[x + 1];
                let target_set: usize = self.cells.get(&l1).unwrap().set_id;
                let l2_cell = self.cells.get_mut(&l2).unwrap();
                l2_cell.set_id = target_set;

                if let Some(set) = self.sets.get_mut(&target_set) {
                    set.insert(l2);
                }
                if let Some(cell) = self.cells.get(&l2) {
                    // Remove l2 from previous set
                    if let Some(set) = self.sets.get_mut(&cell.set_id) {
                        set.remove(&l2);
                    }
                }
            }
        }
    }

    pub fn print_row(&self) {
        let mut ceil = String::new();
        let mut floor = String::new();
        let mut vertical = String::new();
        let number_of_digits = get_number_of_digits(self.width * self.iterations, 10);

        for label in self.row.iter() {
            if let Some(cell) = self.cells.get(&label) {
                if cell.walls.contains(&Wall::Top) {
                    ceil.push('-');
                    (0..number_of_digits).for_each(|_| ceil.push('-'));
                    ceil.push('-');
                } else {
                    ceil.push(' ');
                    (0..number_of_digits).for_each(|_| ceil.push(' '));
                    ceil.push(' ');
                }

                if cell.walls.contains(&Wall::Bottom) {
                    floor.push('-');
                    (0..number_of_digits).for_each(|_| floor.push('-'));
                    floor.push('-');
                } else {
                    floor.push(' ');
                    (0..number_of_digits).for_each(|_| floor.push(' '));
                    floor.push(' ');
                }

                if cell.walls.contains(&Wall::Left) && cell.walls.contains(&Wall::Right) {
                    vertical.push('|');
                    let digit_count = get_number_of_digits(cell.set_id, 10);
                    (0..(number_of_digits - digit_count)).for_each(|_| vertical.push(' '));
                    vertical.push_str(&cell.set_id.to_string());
                    vertical.push('|');
                } else if cell.walls.contains(&Wall::Right) {
                    vertical.push(' ');
                    let digit_count = get_number_of_digits(cell.set_id, 10);
                    (0..(number_of_digits - digit_count)).for_each(|_| vertical.push(' '));
                    vertical.push_str(&cell.set_id.to_string());
                    vertical.push('|');
                } else if cell.walls.contains(&Wall::Left) {
                    vertical.push('|');
                    let digit_count = get_number_of_digits(cell.set_id, 10);
                    (0..(number_of_digits - digit_count)).for_each(|_| vertical.push(' '));
                    vertical.push_str(&cell.set_id.to_string());
                    vertical.push(' ');
                } else {
                    vertical.push(' ');
                    let digit_count = get_number_of_digits(cell.set_id, 10);
                    (0..(number_of_digits - digit_count)).for_each(|_| vertical.push(' '));
                    vertical.push_str(&cell.set_id.to_string());
                    vertical.push(' ');
                }
            }

            vertical.push(' ');
            ceil.push(' ');
            floor.push(' ');
        }

        println!("{}", ceil);
        println!("{}", vertical);
        println!("{}", floor);
    }
}

fn get_number_of_digits(num: usize, base: usize) -> usize {
    let mut num = num;
    let mut count = 0;
    while num != 0 {
        num = num / base;
        count = count + 1;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: usize = 10;

    #[test]
    fn new_maze_test() {
        let maze = MazeBuilder::new(WIDTH);
        assert_eq!(WIDTH + 1, maze.set_cnt);
        assert_eq!(WIDTH, maze.label_cnt);
        assert_eq!(WIDTH, maze.row.len());

        // Initial row
        assert!(maze.cells[&0].walls.contains(&Wall::Left));
        assert!(maze.cells[&0].walls.contains(&Wall::Top));

        for i in 1..WIDTH {
            assert!(maze.cells[&i].walls.contains(&Wall::Top));
        }

        assert!(maze.cells[&(WIDTH - 1)].walls.contains(&Wall::Top));
        assert!(maze.cells[&(WIDTH - 1)].walls.contains(&Wall::Right));
    }

    #[test]
    fn test_copy_row() {
        let maze_bldr = &mut MazeBuilder::new(WIDTH);
        let fst_row = maze_bldr.row.clone();
        let snd_row = maze_bldr.ellers();

        assert_eq!(fst_row.len(), snd_row.len());
    }
}
