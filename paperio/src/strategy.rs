use crate::data::{Cell, Direction, World};

////////////////////////////////////////////////////////////////////////////////

pub struct Strategy {
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
            cur_path: vec![(0, 0); 4],
        }
    }

    fn current_map(my_territory: &[Cell], enemy_territory: &[Vec<Cell>]) -> [[i32; 31]; 31] {
        let mut map = [[1; 31]; 31];
        enemy_territory
            .iter()
            .flat_map(|inner| inner.iter())
            .for_each(|c| map[c.0 as usize][c.1 as usize] = 5);
        my_territory
            .iter()
            .for_each(|c| map[c.0 as usize][c.1 as usize] = 0);
        map
    }

    pub fn on_tick(&mut self, world: World) -> Direction {
        let me = world.me().position.to_cell();
        eprintln!("{:?} - {:?}", me, self.cur_path);
        let my_territory: Vec<Cell> = world.me().territory.iter().map(|p| p.to_cell()).collect();
        let enemy_cells: Vec<Cell> = world
            .iter_enemies()
            .map(|(_, e)| e.position.to_cell())
            .collect();
        let enemy_territory: Vec<Vec<Cell>> = world
            .iter_enemies()
            .map(|(_, e)| e.territory.iter().map(|p| p.to_cell()).collect())
            .collect();
        let map = Self::current_map(&my_territory, &enemy_territory);
        if my_territory.iter().any(|c| c.0 == me.0 && c.1 == me.1) {
            self.cur_path.clear();
            let mut rectangle_scores = Vec::<(i32, Vec<(i32, i32)>)>::new();
            let mut corners = [(0, 0); 31 * 31 - 31 - 30];
            let mut it: usize = 0;
            for i in 0..31 {
                for j in 0..31 {
                    if i != me.0 && j != me.1 {
                        corners[it] = (i, j);
                        it += 1;
                    }
                }
            }
            for c in corners.iter() {
                let min_x: i32;
                let max_x: i32;
                let min_y: i32;
                let max_y: i32;
                if c.0 < me.0 {
                    min_x = c.0;
                    max_x = me.0;
                } else {
                    min_x = me.0;
                    max_x = c.0;
                }
                if c.1 < me.1 {
                    min_y = c.1;
                    max_y = me.1;
                } else {
                    min_y = me.1;
                    max_y = c.1;
                }
                let mut perimeter: Vec<(i32, i32)> = vec![];
                for i in min_x..=max_x {
                    perimeter.push((i, min_y));
                    perimeter.push((i, max_y))
                }
                for i in min_y + 1..=max_y - 1 {
                    perimeter.push((min_x, i));
                    perimeter.push((max_x, i));
                }
                let perimeter_len: i32 = perimeter.len() as i32;
                let min_distance_to_perimeter: i32 = perimeter
                    .iter()
                    .map(|perim_cell| {
                        enemy_cells
                            .iter()
                            .map(|enemy_cell| {
                                enemy_cell.distance_to(Cell(perim_cell.0, perim_cell.1))
                            })
                            .min()
                            .unwrap()
                    })
                    .min()
                    .unwrap();
                let mut score: i32 = 0;
                for i in min_x..=max_x {
                    for j in min_y..=max_y {
                        score += map[i as usize][j as usize];
                    }
                }
                let danger: i32 = perimeter_len - min_distance_to_perimeter;
                rectangle_scores.push((score - danger * danger, perimeter));
            }
            rectangle_scores.sort_by(|a, b| a.0.cmp(&b.0));
            let (_, perimeter) = rectangle_scores.last().unwrap();
            let min_x = perimeter.iter().map(|(x, _)| x).min().unwrap();
            let max_x = perimeter.iter().map(|(x, _)| x).max().unwrap();
            let min_y = perimeter.iter().map(|(_, y)| y).min().unwrap();
            let max_y = perimeter.iter().map(|(_, y)| y).max().unwrap();
            let mut path_corners = [
                (min_x, max_y),
                (max_x, max_y),
                (min_x, min_y),
                (max_x, min_y),
            ];
            path_corners.sort_by(|a, b| {
                Cell(*a.0, *a.1)
                    .distance_to(me)
                    .cmp(&Cell(*b.0, *b.1).distance_to(me))
            });
            self.cur_path.push((*path_corners[0].0, *path_corners[0].1));
            self.cur_path.push((*path_corners[1].0, *path_corners[1].1));
            self.cur_path.push((*path_corners[3].0, *path_corners[3].1));
            self.cur_path.push((*path_corners[2].0, *path_corners[2].1));
        } else if self.cur_path.last().unwrap().0 == me.0 && self.cur_path.last().unwrap().1 == me.1
        {
            self.cur_path.pop();
        }
        if let Some(d) = world.me().direction {
            if d.opposite()
                == me.direction_to(Cell(
                    self.cur_path.last().unwrap().0,
                    self.cur_path.last().unwrap().1,
                ))
            {
                for n in me.iter_neighbors() {
                    if d.opposite() != me.direction_to(n) {
                        return me.direction_to(n);
                    }
                }
            }
        }
        return me.direction_to(Cell(
            self.cur_path.last().unwrap().0,
            self.cur_path.last().unwrap().1,
        ));
    }
}
