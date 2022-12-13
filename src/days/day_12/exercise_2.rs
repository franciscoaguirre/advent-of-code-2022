use super::grid::Pathfinding;
use super::Grid;

pub fn solve(grid: &Grid) -> usize {
    let possible_start_points = grid.get_all_cells_with_height(0);
    possible_start_points
        .iter()
        .map(|&start_point| {
            let mut pathfinding = Pathfinding::new(start_point);
            pathfinding.run(&grid, grid.end)
        })
        .map(|path| path)
        .map(|path| path.len())
        .filter(|&length| length > 0)
        .min()
        .unwrap()
}
