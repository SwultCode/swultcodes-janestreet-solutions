use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::time::Instant;
use std::sync::RwLock;
use once_cell::sync::Lazy;

// Global lookup table for number factors.
static FACTORS_LOOKUP: Lazy<RwLock<BTreeMap<usize, Vec<usize>>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));

fn get_factors(n: usize) -> Vec<usize> {
    {
        let lookup = FACTORS_LOOKUP.read().unwrap();
        if let Some(factors) = lookup.get(&n) {
            return factors.clone();
        }
    }
    let factors = compute_factors(n);
    {
        let mut lookup = FACTORS_LOOKUP.write().unwrap();
        lookup.insert(n, factors.clone());
    }
    factors
}

fn compute_factors(n: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    result.push(1);
    if n > 1 {
        result.push(n);
    }
    let int_sqrt = (n as f32).sqrt() as usize;
    for i in 2..=int_sqrt {
        if n % i == 0 {
            result.push(i);
            if i != n / i {
                result.push(n / i);
            }
        }
    }

    // Heuristic: sort the factors from biggest to smallest s.t. nodes will take longer steps,
    // reducing their value by a greater amount
    result.sort_by(|a, b| b.cmp(a));
    result
}

fn mirror_bounce(node: &Node, mirror: Option<&Mirror>) -> Vec<[isize; 2]> {
    let node_dir = node.direction.to_vector();
    match mirror.map_or(MirrorDirection::None, |m| m.mirror_direction) {
        MirrorDirection::None => vec![node_dir],
        MirrorDirection::Undecided => vec![[-node_dir[1], node_dir[0]], [node_dir[1], -node_dir[0]]],
        MirrorDirection::Backslash => vec![match node.direction {
            Direction::Up => Direction::Left.to_vector(),
            Direction::Down => Direction::Right.to_vector(),
            Direction::Left => Direction::Up.to_vector(),
            Direction::Right => Direction::Down.to_vector(),
        }],
        MirrorDirection::Slash => vec![match node.direction {
            Direction::Up => Direction::Right.to_vector(),
            Direction::Down => Direction::Left.to_vector(),
            Direction::Left => Direction::Down.to_vector(),
            Direction::Right => Direction::Up.to_vector(),
        }],
    }
}

fn get_distance_to_bounds(x: &usize, y: &usize, n: &usize, d: &[isize; 2]) -> usize {
    match d {
        [0, 1]  => (n + 2) - y,
        [0, -1] => y + 1,
        [1, 0]  => (n + 2) - x,
        [-1, 0] => x + 1,
        _       => 0,
    }
}

// Mirrors
#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
enum MirrorDirection {
    None,      // Added for no-mirror case
    Undecided,
    Slash,
    Backslash,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Mirror {
    x: usize,
    y: usize,
    mirror_direction: MirrorDirection,
}

impl Mirror {
    fn new(x: usize, y: usize, mirror_direction: MirrorDirection) -> Self {
        Self { x, y, mirror_direction }
    }
}

impl fmt::Display for MirrorDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MirrorDirection::Undecided => write!(f, " x"),
            MirrorDirection::Slash => write!(f, " /"),
            MirrorDirection::Backslash => write!(f, " \\"),
            MirrorDirection::None => write!(f," \\"),
        }
    }
}

// Node Direction
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn to_vector(self) -> [isize; 2] {
        match self {
            Direction::Up => [0, -1],
            Direction::Down => [0, 1],
            Direction::Left => [-1, 0],
            Direction::Right => [1, 0],
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    direction: Direction,
    value: usize,
    move_set: Vec<usize>,
}

impl Node {
    fn new(x: usize, y: usize, direction: Direction, value: usize) -> Self {
        Self {
            x,
            y,
            direction,
            value,
            move_set: Self::get_move_set(value),
        }
    }

    fn get_move_set(value: usize) -> Vec<usize> {
        let factors = get_factors(value);
        factors
    }

    fn generate_move_set(&mut self) {
        self.move_set = Self::get_move_set(self.value);
    }

    fn move_to(&mut self, x: usize, y: usize, dir: Direction) {
        let dist = self.x.abs_diff(x) + self.y.abs_diff(y);
        self.x = x;
        self.y = y;
        self.direction = dir;
        self.value /= dist;
        self.generate_move_set();
    }
}

// BoardState Structure
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Board {
    size: usize,
    grid: Vec<Vec<Option<usize>>>, // Each cell stores an index into `mirrors`
    mirrors: Vec<Mirror>,
    nodes: VecDeque<Node>,
}

impl Board {
    fn new(size: usize) -> Self {
        Self {
            size,
            grid: vec![vec![None; size + 2]; size + 2],
            mirrors: Vec::new(),
            nodes: VecDeque::new(),
        }
    }

    // Immutable lookup for a mirror at (x, y)
    fn get_mirror(&self, x: usize, y: usize) -> Option<&Mirror> {
        if let Some(idx) = self.grid[x][y] {
            if idx != 0 {
                return self.mirrors.get(idx - 1);
            }
        }
        None
    }

    // Finds possible moves for a node.
    fn find_possible_moves(&self, node_index: usize) -> Vec<[usize; 2]> {
        let node = self.nodes.get(node_index).unwrap();
        let mut moves: Vec<[usize; 2]> = Vec::new();
        let mirror = self.get_mirror(node.x, node.y);

        let walking_directions = mirror_bounce(node,mirror);

        for d in walking_directions.iter() {
            let dist = get_distance_to_bounds(&node.x, &node.y, &self.size, d);

            for step in 1..dist {
                let nx = (node.x as isize + step as isize * d[0]) as usize;
                let ny = (node.y as isize + step as isize * d[1]) as usize;

                if node.move_set.contains(&step) { // if the node can move to the square
                    if let Some(idx) = self.grid[nx][ny] {
                        if idx == 0 { continue; } // if it's another beam (can't place mirrors on beams)
                    }
                    moves.push([nx, ny]);
                }

                if let Some(idx) = self.grid[nx][ny] {
                    if idx != 0 { // if it's a mirror, stop exploring
                        break;
                    }
                }
            }
        }
        moves
    }

    // Returns a new board with the node at node_index moved to (x, y).
    fn with_moved_node(&self, node_index: usize, x: usize, y: usize) -> Self {
        let mut new_board = self.clone();
        if let Some(node) = new_board.nodes.get_mut(node_index) {
            if let Some(idx) = self.grid[node.x][node.y] {
                if idx > 0 {
                    if new_board.mirrors[idx - 1].mirror_direction == MirrorDirection::Undecided {
                        let node_dir = node.direction;
                        let new_direction = if x < node.x { // Mirror deciding logic
                            match node_dir {
                                Direction::Down => MirrorDirection::Slash,
                                Direction::Up   => MirrorDirection::Backslash,
                                _ => MirrorDirection::Undecided,
                            }
                        } else if x > node.x {
                            match node_dir {
                                Direction::Up   => MirrorDirection::Slash,
                                Direction::Down => MirrorDirection::Backslash,
                                _ => MirrorDirection::Undecided,
                            }
                        } else if y < node.y {
                            match node_dir {
                                Direction::Right => MirrorDirection::Slash,
                                Direction::Left  => MirrorDirection::Backslash,
                                _ => MirrorDirection::Undecided,
                            }
                        } else {
                            match node_dir {
                                Direction::Left  => MirrorDirection::Slash,
                                Direction::Right => MirrorDirection::Backslash,
                                _ => MirrorDirection::Undecided,
                            }
                        };
                        let mut new_mirror = new_board.mirrors[idx - 1].clone();
                        new_mirror.mirror_direction = new_direction;
                        new_board.mirrors[idx - 1] = new_mirror;
                    }
                }
            }

            let new_direction = if x != node.x {
                let range = if x > node.x {
                    ((node.x + 1)..x).collect::<Vec<_>>()
                } else {
                    ((x + 1)..node.x).rev().collect::<Vec<_>>()
                };
                for px in range {
                    new_board.grid[px][node.y] = Some(0);
                }
                if x > node.x { Direction::Right } else { Direction::Left }
            } else {
                let range = if y > node.y {
                    ((node.y + 1)..y).collect::<Vec<_>>()
                } else {
                    ((y + 1)..node.y).rev().collect::<Vec<_>>()
                };
                for py in range {
                    new_board.grid[node.x][py] = Some(0);
                }
                if y > node.y { Direction::Down } else { Direction::Up }
            };

            node.move_to(x, y, new_direction);

            // disallow future mirror placement orthogonal to the new mirror
            if x > 0 && x < new_board.size + 1 && y > 0 && y < new_board.size + 1 {
                if new_board.grid[x][y].is_none() {
                    new_board.mirrors.push(Mirror::new(x, y, MirrorDirection::Undecided));
                    let mirror_index = new_board.mirrors.len();
                    new_board.grid[x][y] = Some(mirror_index);
                    if x > 2 { new_board.grid[x - 1][y] = Some(0); }
                    if x < (new_board.size - 1) { new_board.grid[x + 1][y] = Some(0); }
                    if y > 2 { new_board.grid[x][y - 1] = Some(0); }
                    if y < (new_board.size - 1) { new_board.grid[x][y + 1] = Some(0); }
                }
            } else {
                if node.value == 1 {
                    new_board.grid[x][y] = Some(0);
                    if let Some(index) = new_board.nodes.iter().position(|n| n.x == x && n.y == y) {
                        new_board.nodes.remove(index);
                    }
                }
            }
        }
        new_board
    }

    // Returns a new board with a new node placed.
    fn with_placed_node(&self, x: usize, y: usize, direction: Direction, value: usize) -> Self {
        let mut new_board = self.clone();
        new_board.nodes.push_back(Node::new(x, y, direction, value));
        new_board
    }
}

// Solver Structure: Uses DFS only.
#[derive(Clone, Debug)]
struct Solver {
    board: Board,
}

impl Solver {
    fn new(board: Board) -> Self { Self { board } }

    fn solve(&self) -> Option<Board> { self.dfs(&self.board, 0) }

    // DFS
    fn dfs(&self, board: &Board, depth: usize) -> Option<Board> {
        if board.nodes.is_empty() {
            println!("Solution found at depth {}", depth);
            return Some(board.clone());
        }

        let moves = board.find_possible_moves(0);

        for [nx, ny] in moves {
            let new_board = board.with_moved_node(0, nx, ny);

            if let Some(solution) = self.dfs(&new_board, depth + 1) {
                return Some(solution);
            }
        }

        None
    }

    fn board_traverse(&self, board: &Board, x: usize, y: usize, banned_positions: Vec<[usize; 2]>) -> usize {
        // this traverses the final board and finds the beams original values, much less
        // optimised than the rest of the code because it's run 4N times where N is the side length of the board.

        if banned_positions.contains(&[x, y]) { return 0 }

        let mut result = 1;
        let mut dir: Direction;

        if x == 0 { dir = Direction::Right; }
        else if x == board.size + 1 { dir = Direction::Left; }
        else if y == 0 { dir = Direction::Down; }
        else { dir = Direction::Up; }

        let mut node = Node::new(x, y, dir, 1);

        loop {
            let mirror = board.get_mirror(node.x, node.y);
            let walking_directions = mirror_bounce(&node, mirror);

            for d in walking_directions.iter() {
                let dist = get_distance_to_bounds(&node.x, &node.y, &board.size, d);

                for step in 1..dist {
                    let nx = (node.x as isize + step as isize * d[0]) as usize;
                    let ny = (node.y as isize + step as isize * d[1]) as usize;

                    if let Some(idx) = board.grid[nx][ny] {
                        if idx != 0 {
                            if nx < node.x { dir = Direction::Left; }
                            else if nx > node.x  { dir = Direction::Right; }
                            else if ny < node.y  { dir = Direction::Up; }
                            else { dir = Direction::Down; }
                            node.x = nx;
                            node.y = ny;
                            node.direction = dir;
                            result *= step;
                            break;
                        }
                    }

                    if step == dist-1 { return result*step }
                }
            }
            if node.x == 0 || node.x == board.size + 1 ||
                node.y == 0 || node.y == board.size + 1 {
                break;
            }
        }
        result
    }

    // Utility: prints the board state.
    fn print_state(&self, board: &Board) {
        for j in 0..board.grid[0].len() {
            for i in 0..board.grid.len() {
                let cell = board.grid[i][j];
                if let Some(node) = board.nodes.iter().find(|n| n.x == i && n.y == j) {
                    print!("{:4} ", node.value);
                } else {
                    match cell {
                        Some(mirror_idx) => {
                            if mirror_idx != 0 {
                                print!("  {} ", &board.mirrors[mirror_idx - 1].mirror_direction);
                            } else {
                                print!("     ");
                            }
                        }
                        None => {
                            if (i == 0 || i == board.size + 1) || (j == 0 || j == board.size + 1) {
                                print!("  -  ");
                            } else {
                                print!("[   ]");
                            }
                        }
                    }
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    // let example_board = Board::new(5)
    //     .with_placed_node(3, 0, Direction::Down, 9)
    //     .with_placed_node(3, 6, Direction::Up, 36)
    //     .with_placed_node(0, 4, Direction::Right, 16)
    //     .with_placed_node(6, 2, Direction::Left, 75);

    let board = Board::new(10)
        .with_placed_node(3, 0, Direction::Down, 112)
        .with_placed_node(5, 0, Direction::Down, 48)
        .with_placed_node(6, 0, Direction::Down, 3087)
        .with_placed_node(7, 0, Direction::Down, 9)
        .with_placed_node(10, 0, Direction::Down, 1)
        .with_placed_node(11, 2, Direction::Left, 4)
        .with_placed_node(11, 3, Direction::Left, 27)
        .with_placed_node(11, 7, Direction::Left, 16)
        .with_placed_node(1, 11, Direction::Up, 2025)
        .with_placed_node(4, 11, Direction::Up, 12)
        .with_placed_node(6, 11, Direction::Up, 5)
        .with_placed_node(8, 11, Direction::Up, 405)
        .with_placed_node(5, 11, Direction::Up, 64)
        .with_placed_node(0, 4, Direction::Right, 27)
        .with_placed_node(0, 8, Direction::Right, 12)
        .with_placed_node(0, 9, Direction::Right, 225);

    // This is for the printing score logic
    let mut node_positions: Vec<[usize; 2]> = vec![];
    for p in &board.nodes {
        node_positions.push([p.x,p.y]);
    }

    // Order nodes by number of factors (heuristic).
    let mut ordered_nodes: Vec<_> = board.nodes.iter().collect();
    ordered_nodes.sort_by_key(|node| get_factors(node.value).len());
    let mut new_board = Board::new(board.size);
    for node in ordered_nodes {
        new_board = new_board.with_placed_node(
            node.x,
            node.y,
            node.direction,
            node.value,
        );
    }

    let solver = Solver::new(new_board);

    println!("Initial board state:");
    solver.print_state(&solver.board);

    let start = Instant::now();

    if let Some(solution) = solver.solve() {
        let elapsed = start.elapsed();
        println!("Solution found in {:.2?}:", elapsed);
        solver.print_state(&solution);

        let mut prod = 1;
        let mut total = 0;

        for i in 1..solution.size+1 {
            let n = solver.board_traverse(&solution,i,0, node_positions.clone());
            total += n;
        }
        prod *= total;

        let mut total = 0;
        for i in 1..solution.size+1 {
            let n = solver.board_traverse(&solution,i,solution.size+1, node_positions.clone());
            total += n;
        }
        prod *= total;

        let mut total = 0;
        for i in 1..solution.size+1 {
            let n = solver.board_traverse(&solution,0,i, node_positions.clone());
            total += n;
        }
        prod *= total;

        let mut total = 0;
        for i in 1..solution.size+1 {
            let n = solver.board_traverse(&solution,solution.size+1,i, node_positions.clone());
            total += n;
        }
        prod *= total;

        println!("Final solution: {}",prod);
    } else {
        let elapsed = start.elapsed();
        println!("No solution found after {:.2?}.", elapsed);
    }
}
