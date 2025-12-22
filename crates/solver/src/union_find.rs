use std::collections::HashMap;
use std::hash::Hash;

pub struct UnionFind<T> {
    parent: Vec<usize>,
    rank: Vec<usize>,
    index: HashMap<T, usize>,
    values: Vec<T>, // index -> value
}

impl<T> Default for UnionFind<T>
where
    T: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> UnionFind<T>
where
    T: Eq + Hash + Clone,
{
    fn new() -> Self {
        Self {
            parent: Vec::new(),
            rank: Vec::new(),
            index: HashMap::new(),
            values: Vec::new(),
        }
    }

    fn get_or_insert(&mut self, x: T) -> usize {
        if let Some(&i) = self.index.get(&x) {
            i
        } else {
            let i = self.parent.len();
            self.parent.push(i);
            self.rank.push(0);
            self.index.insert(x.clone(), i);
            self.values.push(x);
            i
        }
    }

    fn find_index(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            let root = self.find_index(self.parent[i]);
            self.parent[i] = root;
        }
        self.parent[i]
    }

    pub fn find(&mut self, x: T) -> usize {
        let i = self.get_or_insert(x);
        self.find_index(i)
    }

    pub fn union(&mut self, x: T, y: T) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);

        if rx == ry {
            return false;
        }

        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }

        true
    }

    /// Returns groups ordered by descending size
    pub fn groups_by_size(&mut self) -> Vec<Vec<T>> {
        // Ensure full path compression
        for i in 0..self.parent.len() {
            let root = self.find_index(i);
            self.parent[i] = root;
        }

        // root -> elements
        let mut groups: HashMap<usize, Vec<T>> = HashMap::new();

        for (i, value) in self.values.iter().cloned().enumerate() {
            let root = self.parent[i];
            groups.entry(root).or_default().push(value);
        }

        let mut result: Vec<Vec<T>> = groups.into_values().collect();

        // Sort by group size (largest first)
        result.sort_by_key(|b| std::cmp::Reverse(b.len()));

        result
    }

    /// Pretty-print groups ordered by size
    pub fn print_groups_by_size(&mut self)
    where
        T: std::fmt::Debug,
    {
        for (i, group) in self.groups_by_size().iter().enumerate() {
            println!("Group {} (size {}): {:?}", i + 1, group.len(), group);
        }
    }
}
