use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone)]
/// A change detecting container for other types
///
/// The `Cd` will contain the type you put in it and will deref to the inner type. The inner
/// type can only be changed with the `update()` function which takes a closure that is given
/// a mutable reference to the inner type.
///
/// Every call to `update()` will clone the inner type before running the closure to modify the
/// type. If the value of the inner type after running the closure is not equal to the value before
/// running the closure, then `is_clean()` will return `false` until the `clean()` function is
/// called.
///
/// The inner type is required to implement `Clone` and `PartialEq`.
pub(crate) struct Cd<T: Clone + PartialEq> {
    /// Whether or not the inner type has been borrowed mutably since the last
    /// `clean()`
    dirty: bool,
    /// The inner type
    inner: T,
}

impl<T: Clone + PartialEq> Cd<T> {
    /// Create a new change detector containing the given type
    pub fn new(inner: T) -> Self {
        Cd {
            // `Cd`'s start off dirty
            dirty: true,
            inner,
        }
    }

    /// Mark this object as "clean". The object will become "dirty" when the
    /// inner type is borrowed mutably
    pub fn clean(&mut self) {
        self.dirty = false;
    }

    /// Returns `true` if the inner type has **not** been modified since the last run of
    /// `clean()`.
    pub fn is_clean(&self) -> bool {
        !self.dirty
    }

    /// Consumes the `Cd` and converts to the inner type
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Update the inner type by passing a closure to do the mutation
    /// 
    /// The return value of `update()` will be the same as the return value of the closure.
    pub fn update<F, U>(&mut self, update_inner: F) -> U
    where
        F: FnOnce(&mut T) -> U,
    {
        // Clone the old value so that we can compare it with the new value
        let old_value = self.inner.clone();
        // Modify the inner value with the provided closure
        let return_value = update_inner(&mut self.inner);

        // If the new value is different
        if old_value != self.inner {
            self.dirty = true;
        }

        return_value
    }
}

impl<T: Clone + PartialEq> Deref for Cd<T> {
    type Target = T;

    /// Dereference to the inner type    
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clone + PartialEq> From<T> for Cd<T> {
    fn from(inner: T) -> Self {
        Cd::new(inner)
    }
}

impl<T: Clone + PartialEq + std::fmt::Debug> std::fmt::Debug for Cd<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
