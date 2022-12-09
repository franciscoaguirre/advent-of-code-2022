#[derive(Debug, Default)]
pub struct Forest {
    cells: Vec<Vec<Tree>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tree {
    height: u32,
}

impl Tree {
    pub fn new(height: u32) -> Self {
        Self { height }
    }
}

impl Forest {
    pub fn add_row(&mut self, row: Vec<Tree>) {
        self.cells.push(row);
    }

    pub fn count_visible_trees(&self) -> usize {
        let mut visible_count = 0;

        for (i, row) in self.cells.iter().enumerate() {
            for (j, tree) in row.iter().enumerate() {
                if i == 0 || i == self.cells.len() - 1 || j == 0 || j == self.cells[0].len() - 1 {
                    continue;
                }

                if self.is_tree_visible(tree, i, j) {
                    visible_count += 1;
                }
            }
        }

        let top_and_bottom = self.cells.len() * 2;
        let left_and_right = self.cells[0].len() * 2;
        let corners_counted_twice = 4;

        visible_count = visible_count + top_and_bottom + left_and_right - corners_counted_twice;

        visible_count
    }

    pub fn get_highest_scenic_score(&self) -> usize {
        let mut highest_scenic_score = 0;

        for (i, row) in self.cells.iter().enumerate() {
            for (j, tree) in row.iter().enumerate() {
                let scenic_score = self.get_scenic_score(tree, i, j);

                if scenic_score > highest_scenic_score {
                    highest_scenic_score = scenic_score;
                }
            }
        }

        highest_scenic_score
    }

    fn is_tree_visible(&self, tree: &Tree, row: usize, column: usize) -> bool {
        let is_visible_left = self.cells[row]
            .iter()
            .enumerate()
            .filter(|(other_column, _)| *other_column < column)
            .all(|(_, other_tree)| other_tree.height < tree.height);

        let is_visible_right = self.cells[row]
            .iter()
            .enumerate()
            .filter(|(other_column, _)| *other_column > column)
            .all(|(_, other_tree)| other_tree.height < tree.height);

        let is_visible_top = self
            .cells
            .iter()
            .enumerate()
            .filter(|(other_row, _)| *other_row < row)
            .all(|(_, current_row)| current_row[column].height < tree.height);

        let is_visible_bottom = self
            .cells
            .iter()
            .enumerate()
            .filter(|(other_row, _)| *other_row > row)
            .all(|(_, current_row)| current_row[column].height < tree.height);

        is_visible_left || is_visible_right || is_visible_top || is_visible_bottom
    }

    fn get_scenic_score(&self, tree: &Tree, row: usize, column: usize) -> usize {
        let count_seen_trees_horizontally = |(trees_seen, done): (usize, bool),
                                             (_, current_tree): (usize, &Tree)|
         -> (usize, bool) {
            if done {
                return (trees_seen, done);
            }

            (trees_seen + 1, current_tree.height >= tree.height)
        };
        let count_seen_trees_vertically = |(trees_seen, done): (usize, bool),
                                           (_, current_row): (usize, &Vec<Tree>)|
         -> (usize, bool) {
            if done {
                return (trees_seen, done);
            }

            (trees_seen + 1, current_row[column].height >= tree.height)
        };

        let trees_seen_left = self.cells[row]
            .iter()
            .enumerate()
            .filter(|(other_column, _)| *other_column < column)
            .rev()
            .fold((0, false), count_seen_trees_horizontally);
        let trees_seen_right = self.cells[row]
            .iter()
            .enumerate()
            .filter(|(other_column, _)| *other_column > column)
            .fold((0, false), count_seen_trees_horizontally);

        let trees_seen_top = self
            .cells
            .iter()
            .enumerate()
            .filter(|(other_row, _)| *other_row < row)
            .rev()
            .fold((0, false), count_seen_trees_vertically);
        let trees_seen_bottom = self
            .cells
            .iter()
            .enumerate()
            .filter(|(other_row, _)| *other_row > row)
            .fold((0, false), count_seen_trees_vertically);

        trees_seen_left.0 * trees_seen_right.0 * trees_seen_top.0 * trees_seen_bottom.0
    }

    #[cfg(test)]
    pub fn cells(&self) -> Vec<Vec<Tree>> {
        self.cells.clone()
    }
}
