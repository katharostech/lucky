use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize, Clone)]
/// A change detecting container for other types
///
/// `Cd` dereferences to its inner type and will automatically mark itself as "dirty" when
/// it is borrowed mutably. This allows you to detect changes to the inner type.
///
/// When a `Cd` is created it is considered "dirty". A dirty `Cd` will stay dirty until you
/// explicitly `clean()` it. A clean `Cd` will stay clean until a mutable dreference is made on
/// the `Cd`.
pub(crate) struct Cd<T> {
    /// Whether or not the inner type has been borrowed mutably since the last
    /// `clean()`
    dirty: bool,
    /// The inner type
    inner: T,
}

impl<T> Cd<T> {
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

    /// Get whether or not this object has been mutably borrowed since the last
    /// run of `clean()`.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Consumes the `Cd` and converts to the inner type
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for Cd<T> {
    type Target = T;

    /// Dereference to the inner type    
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Cd<T> {
    /// Mutably dereference to the inner type and mark type as "dirty"
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;

        &mut self.inner
    }
}

impl<T> From<T> for Cd<T> {
    fn from(inner: T) -> Self {
        Cd::new(inner)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Cd<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
