use std::fs;

// https://doc.rust-lang.org/std/collections/binary_heap/index.html

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use aoc_2023::{grid_down, grid_left, grid_right, grid_up};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: Node,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Node {
    position: (usize, usize),
    direction: Option<Direction>,
    streak: u8,
}

type NeighborFn = fn(&Vec<Vec<u8>>, &Node) -> Vec<(Node, u8)>;

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(
    grid: &Vec<Vec<u8>>,
    start: (usize, usize),
    goal: (usize, usize),
    neighbors: NeighborFn,
) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = HashMap::new();

    let mut heap = BinaryHeap::new();

    let start_node = Node {
        position: start,
        direction: None,
        streak: 0,
    };
    // We're at `start`, with a zero cost
    dist.insert(start_node, 0);
    heap.push(State {
        cost: 0,
        node: start_node,
    });

    let mut res = None;
    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, node }) = heap.pop() {
        if node.position == goal {
            res = match res {
                None => Some(cost),
                Some(x) => Some(x.min(cost)),
            };
        }

        // Important as we may have already found a better way
        if cost > *dist.get(&node).unwrap_or(&usize::MAX) {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (neighbor, neighbor_cost) in neighbors(grid, &node) {
            let next = State {
                cost: cost + neighbor_cost as usize,
                node: neighbor,
            };

            // If so, add it to the frontier and continue
            if next.cost < *dist.get(&next.node).unwrap_or(&usize::MAX) {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(next.node, next.cost);
            }
        }
    }

    res
}

fn neighbors1(grid: &Vec<Vec<u8>>, node: &Node) -> Vec<(Node, u8)> {
    let mut res = vec![];

    if node.direction != Some(Direction::Down)
        && (node.direction != Some(Direction::Up) || node.streak != 3)
    {
        let up = grid_up(grid, node.position);
        if let Some((position, cost)) = up {
            res.push((
                Node {
                    position,
                    direction: Some(Direction::Up),
                    streak: if node.direction == Some(Direction::Up) {
                        node.streak + 1
                    } else {
                        1
                    },
                },
                cost,
            ));
        }
    }

    if node.direction != Some(Direction::Up)
        && (node.direction != Some(Direction::Down) || node.streak != 3)
    {
        let down = grid_down(grid, node.position);
        if let Some((position, cost)) = down {
            res.push((
                Node {
                    position,
                    direction: Some(Direction::Down),
                    streak: if node.direction == Some(Direction::Down) {
                        node.streak + 1
                    } else {
                        1
                    },
                },
                cost,
            ));
        }
    }

    if node.direction != Some(Direction::Right)
        && (node.direction != Some(Direction::Left) || node.streak != 3)
    {
        let left = grid_left(grid, node.position);
        if let Some((position, cost)) = left {
            res.push((
                Node {
                    position,
                    direction: Some(Direction::Left),
                    streak: if node.direction == Some(Direction::Left) {
                        node.streak + 1
                    } else {
                        1
                    },
                },
                cost,
            ));
        }
    }

    if node.direction != Some(Direction::Left)
        && (node.direction != Some(Direction::Right) || node.streak != 3)
    {
        let right = grid_right(grid, node.position);
        if let Some((position, cost)) = right {
            res.push((
                Node {
                    position,
                    direction: Some(Direction::Right),
                    streak: if node.direction == Some(Direction::Right) {
                        node.streak + 1
                    } else {
                        1
                    },
                },
                cost,
            ));
        }
    }

    res
}

fn neighbors2(grid: &Vec<Vec<u8>>, node: &Node) -> Vec<(Node, u8)> {
    let mut res = vec![];

    if node.direction != Some(Direction::Down)
        && (node.direction != Some(Direction::Up) || node.streak != 10)
    {
        let mut steps = 4;
        if node.direction == Some(Direction::Up) {
            steps = 1;
        }
        let mut valid = true;
        let mut cost = 0;
        let mut position = node.position;
        for _ in 0..steps {
            let up = grid_up(grid, position);
            if let Some((next_position, c)) = up {
                cost += c;
                position = next_position;
            } else {
                valid = false;
                break;
            }
        }
        if valid {
            res.push((
                Node {
                    position,
                    direction: Some(Direction::Up),
                    streak: if node.direction == Some(Direction::Up) {
                        node.streak + steps as u8
                    } else {
                        steps as u8
                    },
                },
                cost,
            ));
        }
    }

    if node.direction != Some(Direction::Up)
        && (node.direction != Some(Direction::Down) || node.streak != 10)
    {
        let mut steps = 4;
        if node.direction == Some(Direction::Down) {
            steps = 1;
        }
        let mut valid = true;
        let mut cost = 0;
        let mut position = node.position;
        for _ in 0..steps {
            let down = grid_down(grid, position);
            if let Some((next_position, c)) = down {
                cost += c;
                position = next_position;
            } else {
                valid = false;
                break;
            }
        }
        if valid {
            res.push((
                Node {
                    position,
                    direction: Some(Direction::Down),
                    streak: if node.direction == Some(Direction::Down) {
                        node.streak + steps as u8
                    } else {
                        steps as u8
                    },
                },
                cost,
            ));
        }
    }

    if node.direction != Some(Direction::Right)
        && (node.direction != Some(Direction::Left) || node.streak != 10)
    {
        let mut steps = 4;
        if node.direction == Some(Direction::Left) {
            steps = 1;
        }
        let mut valid = true;
        let mut cost = 0;
        let mut position = node.position;
        for _ in 0..steps {
            let left = grid_left(grid, position);
            if let Some((next_position, c)) = left {
                cost += c;
                position = next_position;
            } else {
                valid = false;
                break;
            }
        }
        if valid {
            res.push((
                Node {
                    position,
                    direction: Some(Direction::Left),
                    streak: if node.direction == Some(Direction::Left) {
                        node.streak + steps as u8
                    } else {
                        steps as u8
                    },
                },
                cost,
            ));
        }
    }

    if node.direction != Some(Direction::Left)
        && (node.direction != Some(Direction::Right) || node.streak != 10)
    {
        let mut steps = 4;
        if node.direction == Some(Direction::Right) {
            steps = 1;
        }
        let mut valid = true;
        let mut cost = 0;
        let mut position = node.position;
        for _ in 0..steps {
            let right = grid_right(grid, position);
            if let Some((next_position, c)) = right {
                cost += c;
                position = next_position;
            } else {
                valid = false;
                break;
            }
        }
        if valid {
            res.push((
                Node {
                    position,
                    direction: Some(Direction::Right),
                    streak: if node.direction == Some(Direction::Right) {
                        node.streak + steps as u8
                    } else {
                        steps as u8
                    },
                },
                cost,
            ));
        }
    }

    res
}

fn part1(input: &str) -> i64 {
    let grid = parse_grid(input);

    let start = (0, 0);
    let goal = (grid.len() - 1, grid[0].len() - 1);

    shortest_path(&grid, start, goal, neighbors1).expect("Should have a path") as i64
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Should be a valid digit") as u8)
                .collect()
        })
        .collect()
}

fn part2(input: &str) -> i64 {
    let grid = parse_grid(input);

    let start = (0, 0);
    let goal = (grid.len() - 1, grid[0].len() - 1);

    shortest_path(&grid, start, goal, neighbors2).expect("Should have a path") as i64
}

fn main() {
    let file_path = "data/day17_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        assert_eq!(part1(input), 102);
    }

    #[test]
    fn test_part2() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

        assert_eq!(part2(input), 94);
    }

    #[test]
    fn test_part2_alt() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";

        assert_eq!(part2(input), 71);
    }
}
