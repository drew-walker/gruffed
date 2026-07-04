use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct PathInterner {
    paths: Vec<PathBuf>,
    index: HashMap<PathBuf, u32>,
}

impl PathInterner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, path: impl Into<PathBuf>) -> u32 {
        let path = path.into();
        if let Some(&idx) = self.index.get(&path) {
            return idx;
        }
        let idx = self.paths.len() as u32;
        self.paths.push(path.clone());
        self.index.insert(path, idx);
        idx
    }

    pub fn get(&self, idx: u32) -> Option<&Path> {
        self.paths.get(idx as usize).map(|p| p.as_path())
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intern_returns_same_id_for_same_path() {
        let mut interner = PathInterner::new();
        let id1 = interner.intern("src/a.ts");
        let id2 = interner.intern("src/a.ts");
        assert_eq!(id1, id2);
    }

    #[test]
    fn intern_returns_different_id_for_different_path() {
        let mut interner = PathInterner::new();
        let id1 = interner.intern("src/a.ts");
        let id2 = interner.intern("src/b.ts");
        assert_ne!(id1, id2);
    }

    #[test]
    fn get_returns_correct_path() {
        let mut interner = PathInterner::new();
        let id = interner.intern("src/a.ts");
        assert_eq!(interner.get(id), Some(Path::new("src/a.ts")));
    }

    #[test]
    fn len_counts_unique_paths() {
        let mut interner = PathInterner::new();
        interner.intern("src/a.ts");
        interner.intern("src/a.ts");
        interner.intern("src/b.ts");
        assert_eq!(interner.len(), 2);
    }
}
