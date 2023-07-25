use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[cfg(test)]
mod tests;

#[derive(Default, Debug)]
pub struct EqGraph<V: Sized + Clone + Ord + Eq + Hash> {
    // invariant: self.decreasing(). It follows this is normalzing.
    // Intuitively, because self.parent_of is functional there can be no forks.
    parent_of: HashMap<V, V>,
}

impl<V: Sized + Clone + Ord + Eq + Hash> EqGraph<V> {
    pub fn decreasing(&self) -> bool {
        self.parent_of.iter().all(|(k, v)| k > v)
    }
    pub fn relate(&mut self, a: V, b: V) {
        if a == b {
            // optimization: early exit
            return;
        }
        let ra = self.root_of(&a);
        let rb = self.root_of(&b);
        let [rless, rmore] = if ra == rb {
            // nothing to do here
            return;
        } else if ra < rb {
            [ra, rb]
        } else {
            [rb, ra]
        };
        self.parent_of.insert(rmore.clone(), rless.clone());
    }
    pub fn root_of<'a: 'b, 'b>(&'a self, mut v: &'b V) -> &'b V {
        while let Some(parent) = self.parent_of.get(v) {
            v = parent;
        }
        v
    }
    pub fn flatten(&mut self) {
        let keys: Vec<_> = self.parent_of.keys().cloned().collect();
        for key in keys {
            let root = self.root_of(&key);
            if root != &key {
                let root = root.clone();
                self.parent_of.insert(key, root);
            }
        }
    }
    pub fn iter_parents(&self) -> impl Iterator<Item = (&V, &V)> {
        self.parent_of.iter()
    }
    pub fn iter_roots(&self) -> impl Iterator<Item = (&V, &V)> {
        self.parent_of.keys().map(|x| (x, self.root_of(x)))
    }
    pub fn equivalents<'a>(&'a self, v: &'a V) -> impl Iterator<Item = &'a V> {
        let root = self.root_of(v);
        self.parent_of
            .values()
            .filter(move |v| self.root_of(v) == root)
    }
    pub fn parents_to_children(&self) -> HashMap<V, HashSet<V>> {
        let mut ret: HashMap<V, HashSet<V>> = Default::default();
        for (child, parent) in self.iter_parents() {
            let set = ret.entry(parent.clone()).or_default();
            set.insert(child.clone());
        }
        ret
    }
}
