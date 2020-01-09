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

#[derive(Debug)]
struct MazeBuilder {
    set_to_cells: HashMap<usize, HashSet<usize>>,
    cells: HashMap<usize, Cell>,
    width: usize,
    set_cnt: usize,
    label_cnt: usize,
    row: Vec<usize>,
}

impl MazeBuilder {
    fn ellers(&mut self) -> &Vec<usize> {
        let row = &mut self.row;
        let new_row = row.clone();

        self.row = new_row;
        &self.row
    }

    fn new(width: usize) -> MazeBuilder {
        let mut maze_bldr = MazeBuilder {
            set_to_cells: HashMap::new(),
            cells: HashMap::new(),
            width: width,
            set_cnt: 0,
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
                .set_to_cells
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

        maze_bldr.generate_vertical_walls();
        maze_bldr.generate_bottom_walls();

        maze_bldr
    }

    fn generate_bottom_walls(&mut self) {
        // TODO merge with generate vertical walls to avoid repeating
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
            if let Some(set) = self.set_to_cells.get(&set_label) {
                for cell_label in set {}
            }
        }
    }

    fn generate_vertical_walls(&mut self) {
        // TODO may need to account for existing walls?
        for x in 1..self.width - 1 {
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

                if let Some(set) = self.set_to_cells.get_mut(&target_set) {
                    set.insert(l2);
                }
                if let Some(cell) = self.cells.get(&l2) {
                    // Remove l2 from previous set
                    if let Some(set) = self.set_to_cells.get_mut(&cell.set_id) {
                        set.remove(&l2);
                    }
                }
            }
        }
    }
}

fn main() {
    let maze = MazeBuilder::new(10);
    println!("{:?}", maze.row);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_maze_test() {
        const WIDTH: usize = 10;
        let maze = MazeBuilder::new(WIDTH);
        assert_eq!(WIDTH, maze.set_cnt);
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
}
