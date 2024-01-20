use crate::data::{Cell, Direction, World};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

////////////////////////////////////////////////////////////////////////////////

const EMPTY_TERRITORY: usize = 1;
const ENEMY_TERRITORY: usize = 5;

#[derive(Clone, Debug)]
struct Rectangle(Cell, Cell);

impl Rectangle {
    fn cells(&self) -> HashSet<Cell> {
        let mut cell_set = HashSet::new();

        for x in min(self.0 .0, self.1 .0)..=max(self.0 .0, self.1 .0) {
            for y in min(self.0 .1, self.1 .1)..=max(self.0 .1, self.1 .1) {
                cell_set.insert(Cell(x, y));
            }
        }
        cell_set
    }

    fn perimeter(&self) -> Vec<Cell> {
        let mut perimeter: Vec<Cell> = Vec::new();

        let start_x = self.0 .0;
        let start_y = self.0 .1;

        let finish_x = self.1 .0;
        let finish_y = self.1 .1;

        let x_iter: Vec<i32> = if start_x > finish_x {
            (finish_x..=start_x).rev().collect()
        } else {
            (start_x..=finish_x).collect()
        };

        let y_iter: Vec<i32> = if start_y > finish_y {
            (finish_y..=start_y).rev().collect()
        } else {
            (start_y..=finish_y).collect()
        };

        for y in y_iter.clone() {
            perimeter.push(Cell(start_x, y));
        }

        for x in x_iter.clone().iter().skip(1) {
            perimeter.push(Cell(*x, finish_y));
        }

        for y in y_iter.iter().rev().skip(1) {
            perimeter.push(Cell(finish_x, *y));
        }

        for x in x_iter.iter().rev().skip(1) {
            perimeter.push(Cell(*x, start_y));
        }
        perimeter
    }
}

pub struct Strategy {
    started_move_perimeter: bool,
    best_rectangle: Option<Rectangle>,
}

impl Default for Strategy {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy {
    pub fn new() -> Self {
        Self {
            started_move_perimeter: false,
            best_rectangle: None,
        }
    }

    fn move_perimeter(&mut self, world: &World) -> Option<Direction> {
        let me = world.me();
        let my_cell = me.position.to_cell();

        let rectangle = self.best_rectangle.as_ref().unwrap();

        if my_cell == rectangle.0 && self.started_move_perimeter {
            return None;
        }

        if self.started_move_perimeter {
            let next_cell_ind = rectangle
                .perimeter()
                .iter()
                .position(|x| x.0 == my_cell.0 && x.1 == my_cell.1)
                .unwrap()
                + 1;
            Some(
                my_cell.direction_to(
                    *rectangle
                        .perimeter()
                        .get(next_cell_ind)
                        .expect("Cell expected"),
                ),
            )
        } else {
            self.started_move_perimeter = true;
            Some(my_cell.direction_to(*rectangle.perimeter().get(1).unwrap()))
        }
    }

    pub fn on_tick(&mut self, world: World) -> Direction {
        if self.best_rectangle.is_some() {
            if let Some(direction) = self.move_perimeter(&world) {
                return direction;
            }
        }
        self.best_rectangle = self.get_best_score_rectangle(&world);
        self.started_move_perimeter = false;
        self.on_tick(world)
    }

    fn evaluate_rectangle(rectangle: &Rectangle, world: &World) -> usize {
        let my_territory: HashSet<Cell> =
            world.me().territory.iter().map(|x| x.to_cell()).collect();

        let my_territory_in_rectangle = my_territory
            .intersection(&rectangle.cells())
            .collect::<HashSet<_>>()
            .len();

        let mut enemy_territory: HashSet<Cell> = HashSet::new();
        let mut min_dist: usize = usize::MAX;

        for (_enemy_id, enemy) in world.iter_enemies() {
            if rectangle.cells().contains(&enemy.position.to_cell()) {
                return 0_usize;
            }

            enemy_territory = enemy_territory
                .union(
                    &rectangle
                        .cells()
                        .intersection(&enemy.territory.iter().map(|x| x.to_cell()).collect())
                        .copied()
                        .collect(),
                )
                .copied()
                .collect::<HashSet<Cell>>();

            for cell in rectangle.cells().iter() {
                min_dist = min(
                    min_dist,
                    cell.distance_to(enemy.position.to_cell()) as usize,
                );
            }
        }
        ((rectangle.cells().len() - enemy_territory.len() - my_territory_in_rectangle)
            * EMPTY_TERRITORY
            + enemy_territory.len() * ENEMY_TERRITORY)
            - min_dist.pow(2)
    }

    fn get_best_score_rectangle(&self, world: &World) -> Option<Rectangle> {
        let me = world.me();
        let me_position = me.position.to_cell();

        let me_x = me_position.0;
        let me_y = me_position.1;

        let mut max_score = 0;
        let mut score_rectangle_map: HashMap<usize, Rectangle> = HashMap::new();

        for world_cell in world.iter_cells() {
            if world_cell.0 == me_x {
                continue;
            }
            if world_cell.1 == me_y {
                continue;
            }

            for (_enemy_id, enemy) in world.iter_enemies() {
                if enemy.position.to_cell().0 == me_x && enemy.position.to_cell().1 == me_y {
                    continue;
                }
            }

            let rectangle_to_evaluate = Rectangle(me_position, world_cell);
            let rectangle_score = Strategy::evaluate_rectangle(&rectangle_to_evaluate, world);

            score_rectangle_map.insert(rectangle_score, rectangle_to_evaluate);

            max_score = max(max_score, rectangle_score);
        }
        score_rectangle_map.get(&max_score).cloned()
    }
}
