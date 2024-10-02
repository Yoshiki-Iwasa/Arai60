// Step4

pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        use std::ops::ControlFlow;
        const OBSTACLE: i32 = 1;
        assert!(!obstacle_grid.is_empty());
        assert!(!obstacle_grid[0].is_empty());

        let y_len = obstacle_grid.len();
        let x_len = obstacle_grid[0].len();

        // 1 row
        let mut num_of_paths = vec![0; x_len];

        (0..x_len).try_for_each(|x| {
            if obstacle_grid[0][x] == OBSTACLE {
                return ControlFlow::Break(());
            }
            num_of_paths[x] = 1;
            ControlFlow::Continue(())
        });

        (1..y_len).for_each(|y| {
            (0..x_len).for_each(|x| {
                if obstacle_grid[y][x] == OBSTACLE {
                    num_of_paths[x] = 0;
                    return;
                }
                if x != 0 {
                    num_of_paths[x] += num_of_paths[x - 1]
                }
            });
        });

        num_of_paths[x_len - 1]
    }
}
