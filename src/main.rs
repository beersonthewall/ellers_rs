use rand::random;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

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

#[derive(Debug)]
struct MazeBuilder {
    sets: HashMap<usize, HashSet<usize>>,
    cells: HashMap<usize, Cell>,
    width: usize,
    set_cnt: usize,
    label_cnt: usize,
    row: Vec<usize>,
}

impl MazeBuilder {
    /// Eller's algorithm:
    /// Copy Previous row to next_row
    /// 1) remove right walls.
    /// 2) if cell.walls.contains(Wall::Bottom) set_id = 0;
    /// 3) remove bottom walls
    /// 4) cells without a set get their own unique set
    /// 5) randomly add right walls, merging sets as appropriate (when not adding a wall)
    ///    If two adjacent cells are in the same set, we must add a wall
    /// 6) randomly add bottom walls, each set must have a down-passage

    /// Completing the maze.
    /// 1) create a normal row, except each cell has a bottom wall
    /// 2) remove walls between cells that are members of different sets
    ///    union sets until all cells are members of the same set.
    ///
    /// Returns a vector of cell labels.
    fn ellers(&mut self) -> &Vec<usize> {
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
            let sets = &mut self.sets;

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

                // Use flags to avoid two mutable references.
                if merge && current_set_id != 0 {
                    let next_cell = cells.get_mut(&next_label).unwrap();
                    if let Some(old_set) = sets.get_mut(&next_cell.set_id) {
                        old_set.remove(&next_cell.label);
                    }
                    next_cell.set_id = current_set_id;
                    if let Some(new_set) = sets.get_mut(&current_set_id) {
                        new_set.insert(next_cell.label);
                    }
                }

                if add_left {
                    if let Some(cell) = cells.get_mut(&next_label) {
                        cell.walls.insert(Wall::Left);
                    }
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
        self.init_bottom_walls();
        &self.row
    }

    fn end(&mut self) {
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

    fn new(width: usize) -> MazeBuilder {
        let mut maze_bldr = MazeBuilder {
            sets: HashMap::new(),
            cells: HashMap::new(),
            width: width,
            set_cnt: 1,
            label_cnt: 0,
            row: Vec::new(),
        };

        // Generate the initial row and put each cell into it's own set.
        while maze_bldr.label_cnt < maze_bldr.width {
            maze_bldr.cells.insert(
                maze_bldr.label_cnt,
                Cell {
                    walls: HashSet::new(),
                    label: maze_bldr.label_cnt,
                    set_id: maze_bldr.set_cnt,
                },
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
        maze_bldr.init_bottom_walls();

        maze_bldr
    }

    fn init_bottom_walls(&mut self) {
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

    fn print_row(&self) {
        let mut ceil = String::new();
        let mut floor = String::new();
        let mut vertical = String::new();

        for label in self.row.iter() {
            if let Some(cell) = self.cells.get(&label) {
                if cell.walls.contains(&Wall::Top) {
                    ceil.push('-');
                    ceil.push('-');
                    ceil.push('-');
                } else {
                    ceil.push(' ');
                    ceil.push(' ');
                    ceil.push(' ');
                }

                if cell.walls.contains(&Wall::Bottom) {
                    floor.push('-');
                    floor.push('-');
                    floor.push('-');
                } else {
                    floor.push(' ');
                    floor.push(' ');
                    floor.push(' ');
                }

                if cell.walls.contains(&Wall::Left) && cell.walls.contains(&Wall::Right) {
                    vertical.push('|');
                    vertical.push_str(&cell.set_id.to_string());
                    vertical.push('|');
                } else if cell.walls.contains(&Wall::Right) {
                    vertical.push(' ');
                    vertical.push_str(&cell.set_id.to_string());
                    vertical.push('|');
                } else if cell.walls.contains(&Wall::Left) {
                    vertical.push('|');
                    vertical.push_str(&cell.set_id.to_string());
                    vertical.push(' ');
                } else {
                    vertical.push(' ');
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

fn print_help() {
    println!("Eller's maze generation algorithm implementation.");
    println!("");
    println!("USAGE:");
    println!("      gen [width] [iterations]");
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-help") {
        print_help();
    }
    if args.len() != 3 {
        println!("Error");
    }
    let width = usize::from_str_radix(&args[1], 10).unwrap();
    let iterations = usize::from_str_radix(&args[2], 10).unwrap();

    let mut builder = MazeBuilder::new(width);

    for _ in 0..iterations - 1 {
        builder.ellers();
        builder.print_row();
    }
    builder.end();
    builder.print_row();
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

        for i in 0..WIDTH {
            let fst_row_cell = maze_bldr.cells.get(&i).unwrap();
            let snd_row_cell = maze_bldr.cells.get(&i).unwrap();

            assert_eq!(fst_row_cell.set_id, snd_row_cell.set_id);
            assert_eq!(fst_row_cell.walls, snd_row_cell.walls);
        }
    }
}
