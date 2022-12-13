use super::grid::Pathfinding;
use super::Grid;

pub fn solve(input: &Grid) -> usize {
    let start_node = input.start;
    let end_node = input.end;
    let mut pathfinding = Pathfinding::new(start_node);
    let shortest_path = pathfinding.run(&input, end_node);
    // let shortest_path = pathfinding.find_shortest_path(end_node);
    shortest_path.len()
}
