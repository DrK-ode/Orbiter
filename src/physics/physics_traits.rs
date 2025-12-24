use std::{
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
enum ValueWithOptionalBuffer<T: Copy> {
    UnBufferedValue(T),
    BufferedValue(BufferedValue<T>),
}

impl<T: Copy + Default> Default for ValueWithOptionalBuffer<T> {
    fn default() -> Self {
        ValueWithOptionalBuffer::UnBufferedValue(T::default())
    }
}

#[derive(Debug, Default, Reflect)]
pub struct PropertyValue<T: Copy> {
    value: ValueWithOptionalBuffer<T>,
}

impl<T: Copy> PropertyValue<T> {
    pub fn new(value: T) -> PropertyValue<T> {
        Self {
            value: ValueWithOptionalBuffer::UnBufferedValue(value),
        }
    }
    pub fn new_with_buffer(value: T) -> PropertyValue<T> {
        Self {
            value: ValueWithOptionalBuffer::BufferedValue(BufferedValue {
                current: value,
                buffer: value,
            }),
        }
    }
    pub fn get_value(&self) -> T {
        *(*self)
    }
    pub fn set_value(&mut self, value: T) -> &mut Self {
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
}

impl<T: Copy + Add<Output = T>> PropertyValue<T> {
    pub fn add_assign(&mut self, value: T) {
        self.set_value(**self + value);
    }
}

impl<T: Copy> Deref for PropertyValue<T> {
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
                buffered_value.buffer * (1. - fraction) + buffered_value.buffer * fraction
            },
        }
    }
}

impl PropertyValue<Dir2> {
    pub fn estimate(&self, fraction: f32) -> Dir2 {
        match self.value {
            ValueWithOptionalBuffer::UnBufferedValue(unbuffered_value) => {
                warn!("Calling Property<f32>::estimate on mono-valued property.");
                unbuffered_value
            },
            ValueWithOptionalBuffer::BufferedValue(ref buffered_value) => {
                buffered_value.buffer.slerp(buffered_value.current, fraction)
            },
        }
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
