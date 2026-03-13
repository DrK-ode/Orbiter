use bevy::prelude::*;
use std::collections::HashMap;

use crate::game::util::granular_grid::{GranularGrid, GridValue};

#[derive(Debug)]
pub enum GridError {
    GridExists((i32, i32)),
    GridNonExistent((i32, i32)),
}

#[derive(Debug, Reflect)]
pub struct DynamicGrid<T: GridValue> {
    grid_side_length: f32,
    granular_grids:   HashMap<(i32, i32), GranularGrid<T>>,
}

impl<T: GridValue> Default for DynamicGrid<T> {
    fn default() -> Self {
        Self {
            grid_side_length: 1.,
            granular_grids:   Default::default(),
        }
    }
}

impl<T: GridValue> DynamicGrid<T> {
    pub fn new(grid_side_length: f32) -> Self {
        Self {
            grid_side_length,
            granular_grids: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.granular_grids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.granular_grids.is_empty()
    }

    fn sub_grid_index_at(&self, position: Vec2) -> (i32, i32) {
        let half_length = 0.5 * self.grid_side_length;
        let ix = ((position.x + position.x.signum() * half_length) / self.grid_side_length) as i32;
        let iy = ((position.y + position.y.signum() * half_length) / self.grid_side_length) as i32;
        (ix, iy)
    }

    fn sub_grid_center(&self, index: &(i32, i32)) -> Vec2 {
        Vec2::new(index.0 as f32, index.1 as f32) * self.grid_side_length
    }

    fn sub_grid_area(&self, index: &(i32, i32)) -> Rect {
        let center = self.sub_grid_center(index);
        Rect::from_center_size(center, Vec2::splat(self.grid_side_length))
    }

    pub fn value_at(&self, position: Vec2) -> T {
        let sub_grid_index = self.sub_grid_index_at(position);

        match self.granular_grids.get(&sub_grid_index) {
            Some(sub_grid) => sub_grid,
            None => {
                // Find closest grid to extrapolate from, if two are at the same distance, choose
                // the one closest to the origin.
                let sub_grid_index = match self.granular_grids.keys().min_by(|&index1, &index2| {
                    let d1 =
                        (index1.0 - sub_grid_index.0).pow(2) + (index1.1 - sub_grid_index.1).pow(2);
                    let d2 =
                        (index2.0 - sub_grid_index.0).pow(2) + (index2.1 - sub_grid_index.1).pow(2);
                    match d1.cmp(&d2) {
                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => {
                            (index1.0 + index1.1).cmp(&(index2.0 + index2.1))
                        },
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    }
                }) {
                    Some(index) => index,
                    None => {
                        return T::default();
                    },
                };
                // Index will always be valid
                self.granular_grids.get(sub_grid_index).unwrap()
            },
        }
        .value_at(position)
    }

    // Position could be anywhere inside the grid to be created
    pub fn add_grid_at(
        &mut self,
        position: Vec2,
        granularity: u32,
        value_fn: impl Fn(Vec2) -> T,
    ) -> Result<(), GridError> {
        let sub_grid_index = self.sub_grid_index_at(position);
        if self.granular_grids.contains_key(&sub_grid_index) {
            return Err(GridError::GridExists(sub_grid_index));
        }
        let sub_grid_area = self.sub_grid_area(&sub_grid_index);
        let mut sub_grid = GranularGrid::new(sub_grid_area, granularity);
        sub_grid.recalculate_grid(&value_fn);
        self.granular_grids.insert(sub_grid_index, sub_grid);
        Ok(())
    }

    pub fn add_grids_within_range(
        &mut self,
        center: Vec2,
        radius: f32,
        granularity_fn: impl Fn(Vec2) -> u32,
        value_fn: impl Fn(Vec2) -> T,
    ) {
        let position_max = center + Vec2::splat(radius);
        let index_max = self.sub_grid_index_at(position_max);
        let position_min = center - Vec2::splat(radius);
        let index_min = self.sub_grid_index_at(position_min);
        let radius_squared = radius * radius;
        for ix in index_min.0..index_max.0 + 1 {
            for iy in index_min.1..index_max.1 + 1 {
                let index = (ix, iy);
                let sub_grid_area = self.sub_grid_area(&index);
                let closest_corner = Vec2::new(
                    if ix < 0 {
                        sub_grid_area.max
                    }
                    else {
                        sub_grid_area.min
                    }
                    .x,
                    if iy < 0 {
                        sub_grid_area.max
                    }
                    else {
                        sub_grid_area.min
                    }
                    .y,
                );
                if closest_corner.distance_squared(center) <= radius_squared
                    && !self.granular_grids.contains_key(&index)
                {
                    let mut sub_grid = GranularGrid::new(
                        sub_grid_area,
                        granularity_fn(sub_grid_area.center() / self.grid_side_length),
                    );
                    sub_grid.recalculate_grid(&value_fn);
                    self.granular_grids.insert(index, sub_grid);
                }
            }
        }
    }

    pub fn remove_grid_at(&mut self, position: Vec2) -> Result<(), GridError> {
        let sub_grid_index = self.sub_grid_index_at(position);
        match self.granular_grids.remove(&sub_grid_index) {
            Some(_) => Ok(()),
            None => Err(GridError::GridNonExistent(sub_grid_index)),
        }
    }

    pub fn remove_grids(&mut self, filter: impl Fn(Vec2) -> bool) -> usize {
        let grids_to_remove = self
            .granular_grids
            .keys()
            .filter_map(|&index| {
                if filter(self.sub_grid_center(&index)) {
                    Some(index)
                }
                else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for sub_grid_index in &grids_to_remove {
            self.granular_grids.remove(sub_grid_index);
        }
        grids_to_remove.len()
    }

    pub fn update_grid_granularities(
        &mut self,
        granularity_fn: fn(Vec2) -> u32,
        value_fn: impl Fn(Vec2) -> T,
    ) {
        self.granular_grids.iter_mut().for_each(|(sub_grid_index, sub_grid)| {
            let sub_grid_center =
                Vec2::new(sub_grid_index.0 as f32, sub_grid_index.1 as f32) * self.grid_side_length;
            let diff_granularity =
                sub_grid.update_granularity(granularity_fn(sub_grid_center / self.grid_side_length));
            if diff_granularity > 0 {
                sub_grid.recalculate_grid(&value_fn);
            }
        });
    }

    pub fn update_grid_values(&mut self, value_update_fn: impl Fn(Vec2) -> T) {
        self.granular_grids.values_mut().for_each(|sub_grid| {
            sub_grid.update_grid(&value_update_fn);
        });
    }

    pub fn recalculate_grid_values(&mut self, value_update_fn: impl Fn(Vec2) -> T) {
        self.granular_grids.values_mut().for_each(|sub_grid| {
            sub_grid.recalculate_grid(&value_update_fn);
        });
    }
}

#[cfg(test)]
mod test {
    const MAX_REL_FLOAT_ERR: f32 = 0.001;

    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_sub_grid_area() {
        let grid = DynamicGrid::<f32>::new(1.);

        let area = grid.sub_grid_area(&(0, 0));
        assert_eq!(area.center(), Vec2::ZERO);
        assert_eq!(area.min, Vec2::new(-0.5, -0.5));
        assert_eq!(area.max, Vec2::new(0.5, 0.5));
        let area = grid.sub_grid_area(&(1, 1));
        assert_eq!(area.center(), Vec2::ONE);
        assert_eq!(area.min, Vec2::new(0.5, 0.5));
        assert_eq!(area.max, Vec2::new(1.5, 1.5));
        let area = grid.sub_grid_area(&(-1, -1));
        assert_eq!(area.center(), -Vec2::ONE);
        assert_eq!(area.min, Vec2::new(-1.5, -1.5));
        assert_eq!(area.max, Vec2::new(-0.5, -0.5));
        let area = grid.sub_grid_area(&(-5, 3));
        assert_eq!(area.center(), Vec2::new(-5., 3.));
        assert_eq!(area.min, Vec2::new(-5.5, 2.5));
        assert_eq!(area.max, Vec2::new(-4.5, 3.5));

        let grid = DynamicGrid::<f32>::new(2.);
        let area = grid.sub_grid_area(&(-5, 3));
        assert_eq!(area.center(), 2. * Vec2::new(-5., 3.));
        assert_eq!(area.min, 2. * Vec2::new(-5.5, 2.5));
        assert_eq!(area.max, 2. * Vec2::new(-4.5, 3.5));
    }

    #[test]
    fn test_sub_grid_index_at() {
        let grid = DynamicGrid::<f32>::new(1.);
        assert_eq!(grid.sub_grid_index_at(Vec2::new(0., 0.)), (0, 0));
        assert_eq!(grid.sub_grid_index_at(Vec2::new(1., 0.)), (1, 0));
        assert_eq!(grid.sub_grid_index_at(Vec2::new(0., 1.)), (0, 1));
        assert_eq!(grid.sub_grid_index_at(Vec2::new(1., 1.)), (1, 1));
        assert_eq!(grid.sub_grid_index_at(Vec2::new(-1., 0.)), (-1, 0));
        assert_eq!(grid.sub_grid_index_at(Vec2::new(0., -1.)), (0, -1));
        assert_eq!(grid.sub_grid_index_at(Vec2::new(-1., -1.)), (-1, -1));
        assert_eq!(grid.sub_grid_index_at(Vec2::new(1., -1.)), (1, -1));
        assert_eq!(grid.sub_grid_index_at(Vec2::new(-1., 1.)), (-1, 1));
    }

    #[test]
    fn test_add_and_remove_grid() {
        let mut grid = DynamicGrid::<f32>::new(1.);
        let value_fn = |_position: Vec2| 1.;
        grid.add_grid_at(Vec2::ZERO, 0, value_fn).unwrap();
        assert_eq!(grid.granular_grids.len(), 1);
        assert_eq!(grid.granular_grids.get(&(0, 0)).unwrap().get_grid_area().area(), 1.);
        grid.add_grid_at(Vec2::ZERO, 0, value_fn).expect_err("Should fail");
        grid.add_grid_at(Vec2::new(0.51, 0.49), 0, value_fn).unwrap();
        assert_eq!(grid.granular_grids.len(), 2);
        assert_eq!(grid.granular_grids.get(&(1, 0)).unwrap().get_grid_area().area(), 1.);
        grid.add_grid_at(Vec2::new(-0.51, -0.49), 0, value_fn).unwrap();
        assert_eq!(grid.granular_grids.len(), 3);
        assert_eq!(grid.granular_grids.get(&(-1, 0)).unwrap().get_grid_area().area(), 1.);

        grid.remove_grid_at(Vec2::new(1., 1.)).expect_err("Should fail");
        grid.remove_grid_at(Vec2::new(1., 0.)).unwrap();
        assert_eq!(grid.granular_grids.len(), 2);
        assert_eq!(grid.granular_grids.get(&(0, 0)).unwrap().get_grid_area().area(), 1.);
        assert_eq!(grid.granular_grids.get(&(-1, 0)).unwrap().get_grid_area().area(), 1.);
        grid.remove_grid_at(Vec2::new(-1., 0.)).unwrap();
        assert_eq!(grid.granular_grids.len(), 1);
        assert_eq!(grid.granular_grids.get(&(0, 0)).unwrap().get_grid_area().area(), 1.);
        grid.remove_grid_at(Vec2::new(0., 0.)).unwrap();
        assert_eq!(grid.granular_grids.len(), 0);
    }

    #[test]
    fn test_add_grids_conditionally() {
        let mut grid = DynamicGrid::<f32>::new(1.);
        let granularity_fn = |position: Vec2| position.distance(Vec2::ZERO).trunc() as u32;
        let value_fn = |_position: Vec2| 1.;
        grid.add_grids_within_range(Vec2::ZERO, 2., granularity_fn, value_fn);
        assert_eq!(grid.granular_grids.len(), 21);
        assert_eq!(grid.granular_grids.get(&(0, 0)).unwrap().get_granularity(), 0);
        assert_eq!(grid.granular_grids.get(&(1, 0)).unwrap().get_granularity(), 1);
        assert_eq!(grid.granular_grids.get(&(2, 0)).unwrap().get_granularity(), 2);

        let mut grid = DynamicGrid::<f32>::new(2.);
        let granularity_fn = |position: Vec2| position.distance(Vec2::ZERO).trunc() as u32;
        grid.add_grids_within_range(Vec2::ZERO, 3., granularity_fn, value_fn);
        assert_eq!(grid.granular_grids.len(), 9);

        let mut grid = DynamicGrid::<f32>::new(1.);
        let granularity_fn = |position: Vec2| position.distance(Vec2::ONE).trunc() as u32;
        grid.add_grids_within_range(Vec2::ONE, 3., granularity_fn, value_fn);
        assert_eq!(grid.granular_grids.len(), 45);
    }

    #[test]
    fn test_remove_grids_conditionally() {
        let mut grid = DynamicGrid::<f32>::new(1.);
        let granularity_fn = |position: Vec2| position.distance(Vec2::ZERO).trunc() as u32;
        let value_fn = |_position: Vec2| 1.;
        grid.add_grids_within_range(Vec2::ZERO, 3., granularity_fn, value_fn);
        grid.remove_grids(|position: Vec2| position.x.abs() < 0.1);
        assert_eq!(grid.granular_grids.len(), 38);
        assert!(!grid.granular_grids.contains_key(&(0, -3)));
        assert!(!grid.granular_grids.contains_key(&(0, -2)));
        assert!(!grid.granular_grids.contains_key(&(0, -1)));
        assert!(!grid.granular_grids.contains_key(&(0, 0)));
        assert!(!grid.granular_grids.contains_key(&(0, 1)));
        assert!(!grid.granular_grids.contains_key(&(0, 2)));
        assert!(!grid.granular_grids.contains_key(&(0, 3)));
    }

    #[test]
    fn test_get_value() {
        let mut grid = DynamicGrid::<f32>::new(1.);
        let granularity_fn =
            |position: Vec2| (5. - position.distance(Vec2::ZERO).trunc()).max(0.) as u32;
        let value_fn = |position: Vec2| -> f32 { 10. + position.distance(Vec2::ONE) };
        grid.add_grids_within_range(Vec2::ZERO, 1., granularity_fn, value_fn);
        assert_eq!(grid.granular_grids.len(), 9);
        assert_eq!(grid.granular_grids.get(&(0, 0)).unwrap().get_granularity(), 5);
        assert_eq!(grid.granular_grids.get(&(1, 0)).unwrap().get_granularity(), 4);

        assert_relative_eq!(
            grid.value_at(Vec2::new(0., 0.)),
            10. + 2f32.sqrt(),
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(1., 0.)),
            11.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(1., 1.)),
            10.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(0.65, 0.78)),
            10.4134,
            max_relative = MAX_REL_FLOAT_ERR
        );
    }

    #[test]
    fn test_recalculate_grid() {
        let mut grid = DynamicGrid::<f32>::new(1.);
        let granularity_fn = |_position: Vec2| 1_u32;
        let value_fn = |_position: Vec2| -> f32 { 1. };
        grid.add_grids_within_range(Vec2::ZERO, 1., granularity_fn, value_fn);
        let value_fn = |_position: Vec2| -> f32 { 10. };
        grid.recalculate_grid_values(value_fn);
        assert_eq!(grid.granular_grids.len(), 9);
        assert_relative_eq!(
            grid.value_at(Vec2::new(1., 1.)),
            10.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(-0.3, 2.8)),
            10.,
            max_relative = MAX_REL_FLOAT_ERR
        );
    }
}
