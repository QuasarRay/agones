//! A Go slice's JSON-visible nil/allocated distinction, with one value owner.
use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    slice,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GoSlice<T>(Option<Vec<T>>);

impl<T> Default for GoSlice<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> GoSlice<T> {
    pub fn is_nil(&self) -> bool {
        self.0.is_none()
    }
    pub fn push(&mut self, value: T) {
        self.0.get_or_insert_with(Vec::new).push(value);
    }
}

impl<T> From<Vec<T>> for GoSlice<T> {
    fn from(values: Vec<T>) -> Self {
        Self(Some(values))
    }
}

impl<T> FromIterator<T> for GoSlice<T> {
    fn from_iter<I: IntoIterator<Item = T>>(values: I) -> Self {
        Self(Some(values.into_iter().collect()))
    }
}

impl<T> Extend<T> for GoSlice<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, values: I) {
        // Appending no elements must keep a nil slice nil, as Go append does.
        for value in values {
            self.push(value);
        }
    }
}

impl<T> Deref for GoSlice<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.0.as_deref().unwrap_or_default()
    }
}

impl<T> DerefMut for GoSlice<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.0.as_deref_mut().unwrap_or(&mut [])
    }
}

impl<T> IntoIterator for GoSlice<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.unwrap_or_default().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a GoSlice<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut GoSlice<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for GoSlice<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.0.as_ref() == Some(other)
    }
}
