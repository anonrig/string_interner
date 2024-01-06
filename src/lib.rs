use fxhash::{FxBuildHasher, FxHashMap};

#[derive(Default)]
pub struct Intern<'a> {
    data: FxHashMap<&'a str, InternId>,
    list: Vec<String>,
}

pub type InternId = u32;

impl Intern<'_> {
    /// Create a new intern table.
    pub fn new() -> Self {
        Self {
            data: FxHashMap::default(),
            list: Vec::new(),
        }
    }

    /// Create a new intern table with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: FxHashMap::with_capacity_and_hasher(capacity, FxBuildHasher::default()),
            list: Vec::with_capacity(capacity),
        }
    }

    /// Intern a string.
    /// Returns the interned id.
    /// If the string is already interned, returns the existing id.
    /// The string is stored in the intern table for the lifetime of the program.
    /// The id is a 32-bit integer, so there can be at most 2^32 unique strings interned.
    /// If the limit is reached, this function will panic.
    /// The id is guaranteed to be unique for the lifetime of the program.
    ///
    /// ## Examples
    ///
    /// ```
    /// use intern_string::Intern;
    ///
    /// let mut intern = Intern::new();
    /// let id = intern.intern("hello");
    /// assert_eq!(intern.lookup(id), "hello");
    /// ```
    #[inline]
    pub fn intern<V: Into<String> + AsRef<str>>(&mut self, input: V) -> InternId {
        if let Some(&id) = self.data.get(input.as_ref()) {
            return id;
        }

        let owned = input.into();
        let key: *const str = owned.as_str();
        let id = self.list.len() as InternId;
        self.list.push(owned);

        // SAFETY: we can do this because the allocations inside of a String
        // are stable, and so passing ownership to push does not change the
        // address. furthermore, we have no current API that will allow the
        // strings inside data to be modified, and so they will never reallocate
        self.data.insert(unsafe { &*key }, id);
        id
    }

    /// Lookup the interned string by id.
    ///
    /// # Panics
    ///
    /// Panics if the id is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use intern_string::Intern;
    ///
    /// let mut intern = Intern::new();
    /// let id = intern.intern("hello");
    /// assert_eq!(intern.lookup(id), "hello");
    /// ```
    #[inline]
    pub fn lookup(&self, id: InternId) -> &str {
        &self.list[id as usize]
    }

    /// Lookup the interned string by id.
    /// Returns `None` if the id is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use intern_string::Intern;
    ///
    /// let mut intern = Intern::new();
    /// let id = intern.intern("hello");
    /// assert_eq!(intern.try_lookup(id), Some("hello"));
    /// ```
    #[inline]
    pub fn try_lookup(&self, id: InternId) -> Option<&str> {
        self.list.get(id as usize).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interns_and_handles_lookup() {
        let mut interner = Intern::new();
        let id = interner.intern("hello");
        assert_eq!(interner.lookup(id), "hello");
        assert_eq!(interner.try_lookup(id), Some("hello"));
    }

    #[test]
    fn reallocate() {
        let mut interner = Intern::with_capacity(1);
        let id1 = interner.intern("hello");

        // this should reallocate the internal list
        let id2 = interner.intern("world");

        assert_eq!(interner.lookup(id1), "hello");
        assert_eq!(interner.try_lookup(id1), Some("hello"));
        assert_eq!(interner.lookup(id2), "world");
        assert_eq!(interner.try_lookup(id2), Some("world"));
    }
}
