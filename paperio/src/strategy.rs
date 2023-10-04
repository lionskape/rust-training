use crate::data::{Cell, Direction, World};

////////////////////////////////////////////////////////////////////////////////

pub struct Strategy {
    prev_score: i32,
    cur_path: Vec<(i32, i32)>,
}

impl Default for Strategy {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy {
    pub fn new() -> Self {
        Strategy {
            prev_score: 0,
            cur_path: vec![(0, 0); 4],
        }
    }

    pub fn on_tick(&mut self, world: World) -> Direction {
        eprintln!("{}", world.tick_num);
        if world.tick_num == 523 {
            eprintln!();
        }
        if world.me().territory.iter().any(|p| {
            let c = p.to_cell();
            let me = world.me().position.to_cell();
            c.0 == me.0 && c.1 == me.1
        }) {
            self.cur_path.clear();
            eprintln!("on my territory");
            let mut rectangle_scores = Vec::<(i32, Vec<(i32, i32)>)>::new();
            let corners: Vec<(i32, i32)> = world
                .iter_cells()
                .filter(|c| {
                    c.0 != world.me().position.to_cell().0 && c.1 != world.me().position.to_cell().1
                })
                .map(|c| (c.0, c.1))
                .collect();
            for c in corners.iter() {
                let min_x: i32;
                let max_x: i32;
                let min_y: i32;
                let max_y: i32;
                if c.0 < world.me().position.to_cell().0 {
                    min_x = c.0;
                    max_x = world.me().position.to_cell().0;
                } else {
                    min_x = world.me().position.to_cell().0;
                    max_x = c.0;
                }
                if c.1 < world.me().position.to_cell().1 {
                    min_y = c.1;
                    max_y = world.me().position.to_cell().1;
                } else {
                    min_y = world.me().position.to_cell().1;
                    max_y = c.1;
                }
                let full_rectangle = Vec::<(i32, i32)>::from_iter(
                    world
                        .iter_cells()
                        .filter(|cell| {
                            min_x <= cell.0 && cell.0 <= max_x && min_y <= cell.1 && cell.1 <= max_y
                        })
                        .map(|cell| (cell.0, cell.1)),
                );
                let perimeter: Vec<(i32, i32)> = full_rectangle
                    .iter()
                    .filter(|rectl_cell| {
                        rectl_cell.0 == c.0
                            || rectl_cell.0 == world.me().position.to_cell().0
                            || rectl_cell.1 == c.1
                            || rectl_cell.1 == world.me().position.to_cell().1
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
                let mut score: i32 = 0;
                for rect_cell in full_rectangle.iter() {
                    if world
                        .me()
                        .territory
                        .iter()
                        .map(|p| p.to_cell())
                        .any(|my_ter_cell| {
                            my_ter_cell.0 == rect_cell.0 && my_ter_cell.1 == rect_cell.1
                        })
                    {
                        // my territory costs nothing
                        score += 0;
                    } else if world
                        .iter_enemies()
                        .map(|(_, e)| -> Option<Cell> {
                            e.territory.iter().map(|p| p.to_cell()).find(|enemy_cell| {
                                enemy_cell.0 == rect_cell.0 && enemy_cell.1 == rect_cell.1
                            })
                        })
                        .any(|o| o.is_some())
                    {
                        score += 5;
                    } else {
                        // empty territory one point
                        score += 1;
                    }
                }
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
            let mut path_corners = [
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
            self.cur_path.push(path_corners[0]);
            self.cur_path.push(path_corners[1]);
            self.cur_path.push(path_corners[3]);
            self.cur_path.push(path_corners[2]);
            self.prev_score = *score;
        } else {
            eprintln!("NOT on my territory");
            eprintln!("{:?}", self.cur_path);
            if self.cur_path.last().unwrap().0 == world.me().position.to_cell().0
                && self.cur_path.last().unwrap().1 == world.me().position.to_cell().1
            {
                self.cur_path.pop();
            }
        }
        if let Some(d) = world.me().direction {
            if d.opposite()
                == world.me().position.to_cell().direction_to(Cell(
                    self.cur_path.last().unwrap().0,
                    self.cur_path.last().unwrap().1,
                ))
            {
                for n in world.me().position.to_cell().iter_neighbors() {
                    if world.me().direction.unwrap().opposite()
                        != world.me().position.to_cell().direction_to(n)
                    {
                        return world.me().position.to_cell().direction_to(n);
                    }
                }
            }
        }
        return world.me().position.to_cell().direction_to(Cell(
            self.cur_path.last().unwrap().0,
            self.cur_path.last().unwrap().1,
        ));
    }
}
