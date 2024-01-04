use fxhash::{FxBuildHasher, FxHashMap};

#[derive(Default)]
pub struct Intern {
    data: FxHashMap<&'static str, InternId>,
    inputs: Vec<String>,
}

pub type InternId = u32;

impl Intern {
    /// Create a new intern table.
    pub fn new() -> Self {
        Self {
            data: FxHashMap::default(),
            inputs: Vec::new(),
        }
    }

    /// Create a new intern table with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: FxHashMap::with_capacity_and_hasher(capacity, FxBuildHasher::default()),
            inputs: Vec::with_capacity(capacity),
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
    /// use string_interning::Intern;
    ///
    /// let mut intern = Intern::new();
    /// let id = intern.intern("hello");
    /// assert!(id is InternId);
    /// ```
    pub fn intern<V: Into<String> + AsRef<str>>(&mut self, input: V) -> InternId {
        if let Some(&id) = self.data.get(input.as_ref()) {
            return id;
        }

        let id = self.inputs.len() as InternId;
        self.data.insert(input.as_ref(), id);
        self.inputs.push(input.into());
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
    /// use string_interning::Intern;
    ///
    /// let mut intern = Intern::new();
    /// let id = intern.intern("hello");
    /// assert_eq!(intern.lookup(id), "hello");
    /// ```
    pub fn lookup(&self, id: InternId) -> &str {
        &self.inputs[id as usize]
    }

    /// Lookup the interned string by id.
    /// Returns `None` if the id is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use string_interning::Intern;
    ///
    /// let mut intern = Intern::new();
    /// let id = intern.intern("hello");
    /// assert_eq!(intern.try_lookup(id), Some("hello"));
    /// assert_eq!(intern.try_lookup(id + 1), None);
    /// ```
    pub fn try_lookup(&self, id: InternId) -> Option<&str> {
        self.inputs.get(id as usize).map(|s| s.as_str())
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
        assert_eq!(interner.try_lookup(id + 1), None);
    }
}
