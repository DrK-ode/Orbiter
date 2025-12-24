use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Deref},
};

use bevy::prelude::*;

#[derive(Debug, Default, Reflect)]
pub struct Buffered<T: Copy> {
    latest: T,
    previous: T,
}

impl<T: Copy> Buffered<T> {
    pub fn new(value: T) -> Buffered<T> {
        Self {
            latest: value,
            previous: value,
        }
    }
    pub fn value(&self) -> T {
        self.latest
    }
    pub fn assign(&mut self, value: T) -> &mut Self {
        self.previous = self.latest;
        self.latest = value;
        self
    }
}

impl<T: Copy + Add<Output = T>> AddAssign<T> for Buffered<T> {
    fn add_assign(&mut self, value: T) {
        self.assign(**self + value);
    }
}

impl<T: Copy> Deref for Buffered<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.latest
    }
}

impl Buffered<Vec2> {
    pub fn estimate(&self, fraction: f32) -> Vec2 {
        self.previous.lerp(self.latest, fraction)
    }
}

impl Buffered<f32> {
    pub fn estimate(&self, fraction: f32) -> f32 {
        self.previous.lerp(self.latest, fraction)
    }
}

impl Buffered<Dir2> {
    pub fn estimate(&self, fraction: f32) -> Dir2 {
        self.previous.slerp(self.latest, fraction)
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
