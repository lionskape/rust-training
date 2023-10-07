use crate::data::{BonusType, Cell, Direction, World};

////////////////////////////////////////////////////////////////////////////////

pub struct Strategy {
    cur_path: Vec<Cell>,
}

impl Default for Strategy {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy {
    pub fn new() -> Self {
        Strategy {
            cur_path: vec![Cell(0, 0); 4],
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
            let mut rectangle_scores = Vec::<(i32, Vec<Cell>)>::new();
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
                let mut perimeter: Vec<Cell> = vec![];
                for i in min_x..=max_x {
                    perimeter.push(Cell(i, min_y));
                    perimeter.push(Cell(i, max_y))
                }
                for i in min_y + 1..=max_y - 1 {
                    perimeter.push(Cell(min_x, i));
                    perimeter.push(Cell(max_x, i));
                }
                let perimeter_len: i32 = perimeter.len() as i32;
                let min_distance_to_perimeter = perimeter
                    .iter()
                    .flat_map(|&perim_cell| {
                        enemy_cells
                            .iter()
                            .map(move |enemy_cell| enemy_cell.distance_to(perim_cell))
                    })
                    .min();
                let mut ter: i32 = 0;
                for i in min_x..=max_x {
                    for j in min_y..=max_y {
                        ter += map[i as usize][j as usize];
                    }
                }
                let bonus: i32 = world
                    .bonuses
                    .iter()
                    .map(|b| (b.type_, b.position.to_cell()))
                    .filter(|(_, Cell(x, y))| {
                        min_x <= *x && *x <= max_x && min_y <= *y && *y <= max_y
                    })
                    .fold(0, |acc, (type_, _)| {
                        acc + match type_ {
                            BonusType::Nitro | BonusType::Saw => 100,
                            BonusType::Slowdown => 50,
                        }
                    });
                // let line: i32 = perimeter
                //     .iter()
                //     .flat_map(|perim_cell| {
                //         world
                //             .iter_enemies()
                //             .flat_map(|(_, e)| e.lines.iter().map(|p| p.to_cell()))
                //             .filter(|c| c.0 == perim_cell.0 && c.1 == perim_cell.1)
                //     })
                //     .fold(200, |_, _| 0);
                let mut line = 0;
                for perim_cell in perimeter.iter() {
                    for c in world
                        .iter_enemies()
                        .flat_map(|(_, e)| e.lines.iter().map(|p| p.to_cell()))
                    {
                        if c.0 == perim_cell.0 && c.1 == perim_cell.1 {
                            line = 200;
                        }
                    }
                }
                let danger: i32 =
                    perimeter_len - min_distance_to_perimeter.unwrap_or(perimeter_len);
                let score: i32 = ter + line + bonus;
                eprintln!(
                    "score(ter({}) + bonus ({}) + line({}) = {}), danger({})",
                    ter, bonus, line, score, danger,
                );
                rectangle_scores.push((score - danger * danger, perimeter));
            }
            rectangle_scores.sort_by(|a, b| a.0.cmp(&b.0));
            let (_, perimeter) = rectangle_scores.last().unwrap();
            let min_x = perimeter.iter().map(|Cell(x, _)| x).min().unwrap();
            let max_x = perimeter.iter().map(|Cell(x, _)| x).max().unwrap();
            let min_y = perimeter.iter().map(|Cell(_, y)| y).min().unwrap();
            let max_y = perimeter.iter().map(|Cell(_, y)| y).max().unwrap();
            let mut path_corners = [
                Cell(*min_x, *max_y),
                Cell(*max_x, *max_y),
                Cell(*min_x, *min_y),
                Cell(*max_x, *min_y),
            ];
            path_corners.sort_by_key(|a| a.distance_to(me));
            self.cur_path.push(path_corners[0]);
            self.cur_path.push(path_corners[1]);
            self.cur_path.push(path_corners[3]);
            self.cur_path.push(path_corners[2]);
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
