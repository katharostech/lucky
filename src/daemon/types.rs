use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone)]
/// A change detecting container for other types
///
/// The `Cd` will contain the type you put in it and will deref to the inner type. The inner
/// type can only be changed with the `update()` function which takes a closure that is given
/// a mutable reference to the inner type.
///
/// The first call to `update()` since the last clean will clone the inner type before running the
/// closure to modify the type. Any modifications untill the next `clean()` will happen on a clone
/// of the original type. Calls to `is_clean()` will compare the clone of the type to the old
/// version and will return `true` if the clone and the original are the same. This means that if
/// you modify the type and the set it back to what it was previously, `is_clean()` will still
/// return `true`.
///
/// The inner type is required to implement `Clone` and `PartialEq`.
pub(crate) struct Cd<T: Clone + PartialEq> {
    /// The inner type
    inner: T,
    /// The updated inner type if any
    new_inner: Option<T>,
}

impl<T: Clone + PartialEq> Cd<T> {
    /// Create a new change detector containing the given type
    pub fn new(inner: T) -> Self {
        Cd {
            inner,
            new_inner: None,
        }
    }

    /// Make this object clean
    pub fn clean(&mut self) {
        if let Some(new_inner) = self.new_inner.as_mut() {
            // Update inner with the value from `new_inner`
            std::mem::swap(&mut self.inner, new_inner);
            // And delete the old value ( now stored in `new_inner` )
            self.new_inner = None;
        }
    }

    /// Returns `true` if the inner type has **not** been modified since the last run of
    /// `clean()`.
    pub fn is_clean(&self) -> bool {
        // If we have some updated types
        if let Some(new_inner) = self.new_inner.as_ref() {
            // We are clean if the updated type is identical to the old one
            &self.inner == new_inner
        // If we don't have any updated type
        } else {
            // We are clean
            true
        }
    }

    /// Consumes the `Cd` and converts to the inner type
    pub fn into_inner(self) -> T {
        // Return the latest updated inner type if it exists
        if let Some(new_inner) = self.new_inner {
            new_inner
        // Otherwise return the base inner type
        } else {
            self.inner
        }
    }

    /// Update the inner type by passing a closure to do the mutation
    ///
    /// The return value of `update()` will be the same as the return value of the closure.
    pub fn update<F, U>(&mut self, update_inner: F) -> U
    where
        F: FnOnce(&mut T) -> U,
    {
        // If we already have a new inner type
        if let Some(new_inner) = self.new_inner.as_mut() {
            // Update the inner value
            update_inner(new_inner)

        // If there is no new inner type
        } else {
            // Clone the old inner type to the new one
            let mut new_inner = self.inner.clone();

            // Modify the type
            let ret = update_inner(&mut new_inner);

            // Save the new inner type
            self.new_inner = Some(new_inner);

            // Return the closures return value
            ret
        }
    }
}

impl<T: Clone + PartialEq> Deref for Cd<T> {
    type Target = T;

    /// Dereference to the inner type or the new inner type if it exists    
    fn deref(&self) -> &Self::Target {
        if let Some(new_inner) = &self.new_inner {
            new_inner
        } else {
            &self.inner
        }
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
