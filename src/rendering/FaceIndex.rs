use std::fmt::{self, Display, Formatter};
use std::ops::Add;
use bevy::prelude::Component;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
#[derive(Component)]
pub(crate) struct FaceIndex {
    pub i: u16
}