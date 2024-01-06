use fxhash::{FxBuildHasher, FxHashMap};

#[derive(Default)]
pub struct Intern<'a> {
    data: FxHashMap<&'a str, InternId>,
    list: Vec<Box<str>>,
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

        let owned = input.into().into_boxed_str();

        let str_data = owned.as_ptr();
        let str_len = owned.len();

        let id = self.list.len() as InternId;
        self.list.push(owned);

        // SAFETY: we can do this because the allocations inside of a Box<str>
        // are stable, and so passing ownership to push does not change the
        // address.
        //
        // additionally, because we have not touched the string since we created
        // these raw pointers ourselves, we know that it is valid UTF-8 and so
        // can skip that check as well.
        let k =
            unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(str_data, str_len)) };

        self.data.insert(k, id);
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
        self.list.get(id as usize).map(|s| &**s)
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
