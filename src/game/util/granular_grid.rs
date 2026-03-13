use bevy::prelude::*;
use std::{fmt::Debug, ops::Add};

pub trait BiLerp {
    fn bilerp(
        top_left: &Self,
        top_right: &Self,
        bottom_left: &Self,
        bottom_right: &Self,
        s: Vec2,
    ) -> Self;
}

impl BiLerp for f32 {
    fn bilerp(
        top_left: &Self,
        top_right: &Self,
        bottom_left: &Self,
        bottom_right: &Self,
        s: Vec2,
    ) -> Self {
        let y1 = top_left.lerp(*top_right, s.x);
        let y2 = bottom_left.lerp(*bottom_right, s.x);
        y2.lerp(y1, s.y)
    }
}

pub trait GridValue: Add<Self, Output = Self> + BiLerp + Clone + Copy + Debug + Default {}
impl<T: Add<Self, Output = Self> + BiLerp + Clone + Copy + Debug + Default> GridValue for T {}

#[derive(Debug, Reflect)]
pub struct GranularGrid<T: GridValue> {
    sub_divisions: i32,
    grid_area:     Rect,
    grid_delta:    Vec2,
    grid_vector:   Vec<T>,
}

impl<T: GridValue> GranularGrid<T> {
    pub fn new(grid_area: Rect, granularity: u32) -> Self {
        let sub_divisions = 1i32 << granularity;
        Self {
            sub_divisions,
            grid_area,
            grid_delta: (grid_area.max - grid_area.min) / Vec2::splat(sub_divisions as f32),
            grid_vector: vec![T::default(); ((sub_divisions + 1) * (sub_divisions + 1)) as usize],
        }
    }

    pub fn get_grid_area(&self) -> Rect {
        self.grid_area
    }

    pub fn get_granularity(&self) -> u32 {
        self.sub_divisions.ilog2()
    }

    // Will reuse old values if downscaling.
    // Will use default values if upscaling.
    // Returns granularity change
    pub fn update_granularity(&mut self, granularity: u32) -> i32 {
        let sub_divisions = 1i32 << granularity;
        let diff_granularity = granularity as i32 - self.get_granularity() as i32;
        if diff_granularity != 0 {
            let mut new_grid_vector =
                vec![T::default(); ((sub_divisions + 1) * (sub_divisions + 1)) as usize];
            if diff_granularity < 0 {
                let ratio = self.sub_divisions / sub_divisions;
                new_grid_vector.iter_mut().enumerate().for_each(|(new_index, elem)| {
                    let new_indices =
                        Self::get_2d_index_from_sub_divisions(new_index, sub_divisions);
                    let old_indices = new_indices * ratio;
                    let old_index = self.get_storage_index(old_indices);
                    *elem = self.grid_vector[old_index];
                });
            }
            self.sub_divisions = sub_divisions;
            self.grid_delta =
                (self.grid_area.max - self.grid_area.min) / Vec2::splat(sub_divisions as f32);
            self.grid_vector = new_grid_vector;
        }
        diff_granularity
    }

    fn get_storage_index(&self, indices: IVec2) -> usize {
        (indices.y * (self.sub_divisions + 1) + indices.x) as usize
    }

    fn get_2d_index_from_sub_divisions(index: usize, sub_divisions: i32) -> IVec2 {
        IVec2::new(index as i32 % (sub_divisions + 1), index as i32 / (sub_divisions + 1))
    }

    fn get_2d_index(&self, index: usize) -> IVec2 {
        Self::get_2d_index_from_sub_divisions(index, self.sub_divisions)
    }

    fn get_grid_coordinates(&self, coords: Vec2) -> Vec2 {
        (coords - self.grid_area.min) / self.grid_delta
    }

    /// Calculates the grid indices of the vertex that corresponds to the largest coordinates, that
    /// is still smaller than the input coordinates. If an input coordinate is outside the grid area
    /// either 0 or sub_divisions - 1.
    fn floored_grid_indices(&self, rel_coords: Vec2) -> IVec2 {
        rel_coords.as_ivec2().clamp(IVec2::splat(0), IVec2::splat(self.sub_divisions - 1))
    }

    fn grid_element_position(&self, index: usize) -> Vec2 {
        self.grid_area.min + self.grid_delta * self.get_2d_index(index).as_vec2()
    }

    /// Will extrapolate if outside grid
    pub fn value_at(&self, pos: Vec2) -> T {
        let grid_coords = self.get_grid_coordinates(pos);
        let low_indices = self.floored_grid_indices(grid_coords);

        let index_bottom_left = self.get_storage_index(low_indices);
        let index_top_left = self.get_storage_index(low_indices + IVec2::new(0, 1));
        let index_bottom_right = self.get_storage_index(low_indices + IVec2::new(1, 0));
        let index_top_right = self.get_storage_index(low_indices + IVec2::new(1, 1));

        let value_bottom_left = &self.grid_vector[index_bottom_left];
        let value_top_left = &self.grid_vector[index_top_left];
        let value_bottom_right = &self.grid_vector[index_bottom_right];
        let value_top_right = &self.grid_vector[index_top_right];

        let s = grid_coords - low_indices.as_vec2();

        T::bilerp(value_top_left, value_top_right, value_bottom_left, value_bottom_right, s)
    }

    pub fn recalculate_grid(&mut self, value_fn: impl Fn(Vec2) -> T) {
        for index in 0..self.grid_vector.len() {
            let position = self.grid_element_position(index);
            self.grid_vector[index] = value_fn(position);
        }
    }

    pub fn update_grid(&mut self, value_fn: impl Fn(Vec2) -> T) {
        for index in 0..self.grid_vector.len() {
            let position = self.grid_element_position(index);
            let value = self.grid_vector[index] + value_fn(position);
            self.grid_vector[index] = value;
        }
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use super::*;

    const MAX_REL_FLOAT_ERR: f32 = 0.001;

    fn create_test_grid(rect: Rect, granularity: u32) -> GranularGrid<f32> {
        let mut grid = GranularGrid::<f32>::new(rect, granularity);
        let number_of_grid_points_1d = (1 << granularity) + 1;
        let number_of_grid_points = number_of_grid_points_1d * number_of_grid_points_1d;
        assert_eq!(grid.grid_vector.len(), number_of_grid_points as usize);
        for y in 0..number_of_grid_points_1d {
            for x in 0..number_of_grid_points_1d {
                let index = grid.get_storage_index(IVec2 { x, y });
                grid.grid_vector[index] = y as f32 * 100. + x as f32;
            }
        }
        grid
    }

    #[test]
    fn test_grid_creation() {
        let rect = Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(1.));
        let grid = create_test_grid(rect, 1);
        assert_eq!(grid.get_granularity(), 1);
        assert_relative_eq!(grid.grid_delta.x, 1., max_relative = MAX_REL_FLOAT_ERR);
        assert_eq!(grid.grid_vector.len(), 9);
        let rect = Rect::from_center_half_size(Vec2::ONE, Vec2::splat(5.));
        let grid = create_test_grid(rect, 3);
        assert_eq!(grid.get_granularity(), 3);
        assert_relative_eq!(grid.grid_delta.x, 1.25, max_relative = MAX_REL_FLOAT_ERR);
        assert_eq!(grid.grid_vector.len(), 81);
    }

    #[test]
    fn test_grid_interpolation() {
        let grid = create_test_grid(Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(1.)), 2);
        assert_relative_eq!(grid.value_at(Vec2::ZERO), 202., max_relative = MAX_REL_FLOAT_ERR);
        assert_relative_eq!(
            grid.value_at(Vec2::new(0.25, 0.25)),
            252.5,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(-0.25, 0.75)),
            351.5,
            max_relative = MAX_REL_FLOAT_ERR
        );
        let grid = create_test_grid(Rect::from_center_half_size(Vec2::ONE, Vec2::splat(1.)), 2);
        assert_relative_eq!(grid.value_at(Vec2::ONE), 202., max_relative = MAX_REL_FLOAT_ERR);
        assert_relative_eq!(
            grid.value_at(Vec2::ONE + Vec2::new(0.25, 0.25)),
            252.5,
            max_relative = MAX_REL_FLOAT_ERR,
        );
        assert_relative_eq!(
            grid.value_at(Vec2::ONE + Vec2::new(-0.25, 0.75)),
            351.5,
            max_relative = MAX_REL_FLOAT_ERR,
        );
    }

    #[test]
    fn test_grid_extrapolation() {
        let grid = create_test_grid(Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(8.)), 4);
        assert_relative_eq!(
            grid.value_at(Vec2::new(-8., -8.)),
            0.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(-9., -9.)),
            -101.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(8., 8.)),
            1616.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(9., 9.)),
            1717.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(20., 20.)),
            2828.,
            max_relative = MAX_REL_FLOAT_ERR
        );
    }

    #[test]
    fn test_upscaling_granularity() {
        let rect = Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(1.));
        let mut grid = create_test_grid(rect, 1);
        grid.update_granularity(2);
        assert_eq!(grid.grid_area, rect);
        assert_eq!(grid.grid_delta, Vec2::splat(0.5));
        assert_eq!(grid.grid_vector.len(), 25);
        assert!(grid.grid_vector.iter().all(|elem| *elem == 0.));
    }

    #[test]
    fn test_downscaling_granularity() {
        let rect = Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(1.));
        let mut grid = create_test_grid(rect, 3);
        assert_relative_eq!(grid.value_at(Vec2::ZERO), 404., max_relative = MAX_REL_FLOAT_ERR);
        assert_relative_eq!(
            grid.value_at(Vec2::new(-1., 1.)),
            800.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(1., 1.)),
            808.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        grid.update_granularity(2);
        assert_eq!(grid.grid_area, rect);
        assert_eq!(grid.grid_delta, Vec2::splat(0.5));
        assert_eq!(grid.grid_vector.len(), 25);
        assert_relative_eq!(grid.value_at(Vec2::ZERO), 404., max_relative = MAX_REL_FLOAT_ERR);
        assert_relative_eq!(
            grid.value_at(Vec2::new(-1., 1.)),
            800.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(1., 1.)),
            808.,
            max_relative = MAX_REL_FLOAT_ERR
        );
    }

    #[test]
    fn test_recalculate_grid() {
        let rect = Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(1.));
        let mut grid = create_test_grid(rect, 3);
        grid.recalculate_grid(|position| position.element_sum());
        assert_relative_eq!(grid.value_at(Vec2::ZERO), 0., max_relative = MAX_REL_FLOAT_ERR);
        assert_relative_eq!(
            grid.value_at(Vec2::new(-1., 2.)),
            1.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(grid.value_at(Vec2::new(1., 1.)), 2., max_relative = MAX_REL_FLOAT_ERR);
    }

    #[test]
    fn test_update_grid() {
        let rect = Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(1.));
        let mut grid = create_test_grid(rect, 3);
        grid.update_grid(|position| position.element_sum());
        assert_relative_eq!(grid.value_at(Vec2::ZERO), 404., max_relative = MAX_REL_FLOAT_ERR);
        assert_relative_eq!(
            grid.value_at(Vec2::new(-1., 1.)),
            800.,
            max_relative = MAX_REL_FLOAT_ERR
        );
        assert_relative_eq!(
            grid.value_at(Vec2::new(1., 1.)),
            810.,
            max_relative = MAX_REL_FLOAT_ERR
        );
    }
}
