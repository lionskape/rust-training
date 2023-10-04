use crate::data::{Cell, Direction, World};

////////////////////////////////////////////////////////////////////////////////

pub struct Strategy {
    prev_score: i32,
    cur_path: Vec<(i32, i32)>,
}

impl Strategy {
    pub fn new() -> Self {
        Strategy {
            prev_score: 0,
            cur_path: vec![],
        }
    }

    pub fn on_tick(&mut self, world: World) -> Direction {
        if world.me().territory.iter().any(|p| {
            let c = p.to_cell();
            c.0 == world.me().position.0 && c.1 == world.me().position.1
        }) {
            let mut rectangle_scores = Vec::<(i32, Vec<(i32, i32)>)>::new();
            let corners = world
                .iter_cells()
                .filter(|c| c.0 != world.me().position.0 && c.1 != world.me().position.1);
            for c in corners {
                let full_rectangle: Vec<(i32, i32)> = world
                    .iter_cells()
                    .filter(|c| {
                        let min_x: i32;
                        let max_x: i32;
                        let min_y: i32;
                        let max_y: i32;
                        if c.0 < world.me().position.0 {
                            min_x = c.0;
                            max_x = world.me().position.0;
                        } else {
                            min_x = world.me().position.0;
                            max_x = c.0;
                        }
                        if c.1 < world.me().position.1 {
                            min_y = c.1;
                            max_y = world.me().position.1;
                        } else {
                            min_y = world.me().position.1;
                            max_y = c.1;
                        }
                        min_x <= c.0 && c.0 <= max_x && max_y <= c.1 && min_y <= c.1
                    })
                    .map(|c| (c.0, c.1))
                    .collect();
                let perimeter: Vec<(i32, i32)> = full_rectangle
                    .iter()
                    .filter(|rectl_cell| {
                        rectl_cell.0 == c.0
                            || rectl_cell.0 == world.me().position.0
                            || rectl_cell.1 == c.1
                            || rectl_cell.1 == world.me().position.1
                    })
                    .map(|c| (c.0, c.1))
                    .collect();
                let perimeter_len: i32 = perimeter.len() as i32;
                let min_distance_to_perimeter: i32 = perimeter
                    .iter()
                    .map(|perim_cell| -> i32 {
                        world
                            .iter_enemies()
                            .map(|(_, e)| {
                                e.position
                                    .to_cell()
                                    .distance_to(Cell(perim_cell.0, perim_cell.1))
                            })
                            .min()
                            .unwrap()
                    })
                    .min()
                    .unwrap();
                let score: i32 = full_rectangle.len() as i32; // should ignore my territory and cout enemy teritorry
                let danger: i32 = perimeter_len - min_distance_to_perimeter;
                rectangle_scores.push((
                    score - danger * danger,
                    perimeter.iter().map(|c| (c.0, c.1)).collect(),
                ));
            }
            rectangle_scores.sort_by(|a, b| a.0.cmp(&b.0));
            let (score, perimeter) = rectangle_scores.last().unwrap();
            let min_x = perimeter.iter().map(|c| c.0).min().unwrap();
            let max_x = perimeter.iter().map(|c| c.0).max().unwrap();
            let min_y = perimeter.iter().map(|c| c.1).min().unwrap();
            let max_y = perimeter.iter().map(|c| c.1).max().unwrap();
            let ul_cell = world
                .iter_cells()
                .find(|c| c.0 == min_x && c.1 == max_y)
                .unwrap();
            let ur_cell = world
                .iter_cells()
                .find(|c| c.0 == max_x && c.1 == max_y)
                .unwrap();
            let bl_cell = world
                .iter_cells()
                .find(|c| c.0 == min_x && c.1 == min_y)
                .unwrap();
            let br_cell = world
                .iter_cells()
                .find(|c| c.0 == max_x && c.1 == min_y)
                .unwrap();
            let mut path_corners = vec![
                (ul_cell.0, ul_cell.1),
                (ur_cell.0, ur_cell.1),
                (bl_cell.0, bl_cell.1),
                (br_cell.0, br_cell.1),
            ];
            path_corners.sort_by(|a, b| {
                Cell(a.0, a.1)
                    .distance_to(world.me().position.to_cell())
                    .cmp(&Cell(b.0, b.1).distance_to(world.me().position.to_cell()))
            });
            path_corners.reverse();
            path_corners.pop();
            path_corners.reverse();

            'outer: for p in world.me().territory.iter() {
                for c in path_corners.iter() {
                    if c.0 == p.to_cell().0 && c.1 == p.to_cell().1 {
                        self.cur_path.push((c.0, c.1));
                        break 'outer;
                    }
                }
            }
            let mut two_far_corners: Vec<(i32, i32)> = vec![];
            for c in path_corners {
                if c.0 != self.cur_path[0].0 && c.1 != self.cur_path[0].1 {
                    two_far_corners.push(c);
                }
            }
            if two_far_corners[0].0 == self.cur_path[0].0
                && two_far_corners[0].1 == self.cur_path[0].1
            {
                self.cur_path.push(two_far_corners[1]);
                self.cur_path.push(two_far_corners[0]);
            } else if two_far_corners[1].0 == self.cur_path[0].0
                && two_far_corners[1].1 == self.cur_path[0].1
            {
                self.cur_path.push(two_far_corners[0]);
                self.cur_path.push(two_far_corners[1]);
            } else {
                panic!("two_far_corners: two corner candites both match last corner path")
            }
            self.prev_score = *score;
        } else {
            if self.cur_path.last().unwrap().0 == world.me().position.to_cell().0
                && self.cur_path.last().unwrap().1 == world.me().position.to_cell().1
            {
                self.cur_path.pop();
            }
        }
        return world.me().position.to_cell().direction_to(Cell(
            self.cur_path.last().unwrap().0,
            self.cur_path.last().unwrap().1,
        ));
    }
}
