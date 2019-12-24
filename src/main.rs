use rand::random;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Wall {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug)]
struct Cell {
    walls: HashSet<Wall>,
    label: usize,
}

#[derive(Debug)]
struct MazeState {
    sets: HashMap<usize, Vec<usize>>,
    cells: HashMap<usize, usize>,
    width: usize,
    set_cnt: usize,
    label_cnt: usize,
    row: Vec<Cell>,
}

impl MazeState {

    fn ellers(&mut self) -> Vec<Cell> {
        Vec::new()
    }

    fn new(width: usize) -> MazeState {
        let mut maze = MazeState {
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

            maze.row.get_mut(maze.label_cnt).unwrap().walls.insert(Wall::Top);
            let mut lst = maze.sets.entry(maze.set_cnt).or_insert(Vec::new());
            lst.push(maze.label_cnt);
            maze.cells.insert(maze.label_cnt, maze.set_cnt);
            maze.label_cnt += 1;
            maze.set_cnt += 1;
        }

        maze.row.get_mut(0).unwrap().walls.insert(Wall::Left);
        maze.row.get_mut(width - 1).unwrap().walls.insert(Wall::Right);
        maze
    }
}

fn main() {
    let maze = MazeState::new(10);
    println!("{:?}", maze);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_maze_test() {
        const WIDTH: usize = 10;
        let maze = MazeState::new(WIDTH);
        assert_eq!(WIDTH, maze.set_cnt);
        assert_eq!(WIDTH, maze.label_cnt);
        assert_eq!(WIDTH, maze.row.len());

        // Set to cell mapping
        for i in 0..WIDTH {
            assert_eq!(1, maze.sets.get(&i).unwrap().len());
            assert_eq!(i, maze.sets.get(&i).unwrap()[0]);
        }

        // Cell to set mapping
        for i in 0..WIDTH {
            assert_eq!(i, *maze.cells.get(&i).unwrap());
        }

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
