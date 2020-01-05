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
}

#[derive(Debug)]
struct MazeBuilder {
    sets: HashMap<usize, HashSet<usize>>,
    cells: HashMap<usize, usize>,
    width: usize,
    set_cnt: usize,
    label_cnt: usize,
    row: Vec<Cell>,
}

impl MazeBuilder {
    fn ellers(&mut self) -> &Vec<Cell> {
        let row = &mut self.row;
        let new_row = row.clone();

        self.row = new_row;
        &self.row
    }

    fn new(width: usize) -> MazeBuilder {
        let mut maze = MazeBuilder {
            sets: HashMap::new(),
            cells: HashMap::new(),
            width: width,
            set_cnt: 0,
            label_cnt: 0,
            row: Vec::new(),
        };

        // Generate the initial row and put each cell into it's own set.
        while maze.label_cnt < maze.width {
            maze.row.push(Cell {
                walls: HashSet::new(),
                label: maze.label_cnt,
            });

            maze.row
                .get_mut(maze.label_cnt)
                .unwrap()
                .walls
                .insert(Wall::Top);
            let set = maze.sets.entry(maze.set_cnt).or_insert(HashSet::new());
            set.insert(maze.label_cnt);
            maze.cells.insert(maze.label_cnt, maze.set_cnt);
            maze.label_cnt += 1;
            maze.set_cnt += 1;
        }

        maze.row.get_mut(0).unwrap().walls.insert(Wall::Left);
        maze.row
            .get_mut(width - 1)
            .unwrap()
            .walls
            .insert(Wall::Right);

        maze.generate_vertical_walls();
        maze.generate_bottom_walls();

        maze
    }

    fn generate_bottom_walls(&mut self) {}

    fn generate_vertical_walls(&mut self) {
        // TODO may need to account for existing walls?
        for x in 1..self.width - 1 {
            if random() {
                self.row[x].walls.insert(Wall::Right);
                self.row[x + 1].walls.insert(Wall::Left);
            } else {
                let l1 = self.row[x].label;
                let l2 = self.row[x + 1].label;
                let cells = &mut self.cells;
                let target_set = cells.get(&l1).unwrap();

                // Add l2 to target set
                if let Some(set) = self.sets.get_mut(&target_set) {
                    set.insert(l2);
                }
                if let Some(set_id) = cells.get(&l2) {
                    // Remove l2 from previous set
                    if let Some(set) = self.sets.get_mut(&set_id) {
                        set.remove(&l2);
                    }
                }
                // Overwrite previous entry for l2.
                cells.insert(l2, *target_set);
            }
        }
    }
}

fn main() {
    let mut maze = MazeBuilder::new(10);
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
        assert!(maze.row[0].walls.contains(&Wall::Left));
        assert!(maze.row[0].walls.contains(&Wall::Top));

        for i in 1..WIDTH {
            assert!(maze.row[i].walls.contains(&Wall::Top));
        }

        assert!(maze.row[WIDTH - 1].walls.contains(&Wall::Top));
        assert!(maze.row[WIDTH - 1].walls.contains(&Wall::Right));
    }
}
