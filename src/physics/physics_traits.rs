use std::{
    f32::consts::PI,
    marker::PhantomData,
    ops::{Add, Deref},
};

use bevy::prelude::*;

#[derive(Debug, Reflect)]
struct BufferedValue<T> {
    current: T,
    buffer: T,
}

#[derive(Debug, Reflect)]
enum ValueWithOptionalBuffer<T: PropertyPrimitive> {
    UnBufferedValue(T),
    BufferedValue(BufferedValue<T>),
}

pub trait PropertyPrimitive: Add<Output = Self> + Copy + Default {}
impl<T: PropertyPrimitive> Default for ValueWithOptionalBuffer<T> {
    fn default() -> Self {
        ValueWithOptionalBuffer::UnBufferedValue(T::default())
    }
}

impl PropertyPrimitive for f32 {}
impl PropertyPrimitive for Vec2 {}

#[derive(Debug, Default, Reflect)]
pub struct PropertyValue<T: PropertyPrimitive> {
    value: ValueWithOptionalBuffer<T>,
}

pub trait Property<T: PropertyPrimitive> {
    fn new(value: T) -> Self;
    fn new_with_buffer(value: T) -> Self;
    fn get_value(&self) -> T;
    fn set_value(&mut self, value: T) -> &mut Self;
    fn add_assign(&mut self, value: T);
}

impl<T: PropertyPrimitive> Property<T> for PropertyValue<T> {
    fn new(value: T) -> PropertyValue<T> {
        Self {
            value: ValueWithOptionalBuffer::UnBufferedValue(value),
        }
    }
    fn new_with_buffer(value: T) -> PropertyValue<T> {
        Self {
            value: ValueWithOptionalBuffer::BufferedValue(BufferedValue {
                current: value,
                buffer: value,
            }),
        }
    }
    fn get_value(&self) -> T {
        *(*self)
    }
    fn set_value(&mut self, value: T) -> &mut Self {
        match self.value {
            ValueWithOptionalBuffer::UnBufferedValue(ref mut unbuffered_value) => {
                *unbuffered_value = value
            },
            ValueWithOptionalBuffer::BufferedValue(ref mut buffered_value) => {
                buffered_value.buffer = buffered_value.current;
                buffered_value.current = value;
            },
        }
        self
    }
    fn add_assign(&mut self, value: T) {
        self.set_value(**self + value);
    }
}

impl<T: PropertyPrimitive> Deref for PropertyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.value {
            ValueWithOptionalBuffer::UnBufferedValue(ref unbuffered_value) => unbuffered_value,
            ValueWithOptionalBuffer::BufferedValue(ref buffered_value) => &buffered_value.current,
        }
    }
}

impl PropertyValue<Vec2> {
    pub fn estimate(&self, fraction: f32) -> Vec2 {
        match self.value {
            ValueWithOptionalBuffer::UnBufferedValue(unbuffered_value) => {
                warn!("Calling Property<Vec2>::estimate on mono-valued property.");
                unbuffered_value
            },
            ValueWithOptionalBuffer::BufferedValue(ref buffered_value) => {
                buffered_value.buffer.lerp(buffered_value.current, fraction)
            },
        }
    }
}

impl PropertyValue<f32> {
    pub fn estimate(&self, fraction: f32) -> f32 {
        match self.value {
            ValueWithOptionalBuffer::UnBufferedValue(unbuffered_value) => {
                warn!("Calling Property<f32>::estimate on mono-valued property.");
                unbuffered_value
            },
            ValueWithOptionalBuffer::BufferedValue(ref buffered_value) => {
                let mut angle_diff_ratio = (buffered_value.current - buffered_value.buffer) / PI;
                if angle_diff_ratio > 1. {
                    angle_diff_ratio -= 2.
                } else if angle_diff_ratio < -1. {
                    angle_diff_ratio += 2.
                }
                buffered_value.buffer + angle_diff_ratio * fraction
            },
        }
    }
}

#[derive(Debug, Deref, Reflect)]
pub struct AngleLikeValue {
    property: PropertyValue<f32>,
}

impl AngleLikeValue {
    pub fn new(angle: f32) -> AngleLikeValue {
        Self {
            property: PropertyValue::new(angle.rem_euclid(2. * PI)),
        }
    }
    pub fn new_with_buffer(angle: f32) -> Self {
        Self {
            property: PropertyValue::new_with_buffer(angle.rem_euclid(2. * PI)),
        }
    }
    pub fn get_angle(&self) -> f32 {
        self.property.get_value()
    }
    pub fn set_angle(&mut self, angle: f32) -> &mut Self {
        self.property.set_value(angle.rem_euclid(2. * PI));
        self
    }
    pub fn get_vec(&self) -> Vec2 {
        Vec2::from_angle(self.get_angle())
    }
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.set_angle(self.get_angle() + angle);
        self
    }
    pub fn angle_to(&self, angle: f32) -> f32 {
        let mut diff = self.get_angle() - angle;
        if diff > PI {
            diff -= 2. * PI;
        } else if diff < -PI {
            diff += 2. * PI;
        }
        diff
    }
}

#[derive(Debug, Component, Reflect)]
pub struct ValueLimit<T> {
    pub limit: f32,
    phantom: PhantomData<T>,
}

impl<T> ValueLimit<T> {
    pub fn new(limit: f32) -> Self {
        Self {
            limit,
            phantom: Default::default(),
        }
    }
}
