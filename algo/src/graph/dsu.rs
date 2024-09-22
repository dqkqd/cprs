use std::cell::Cell;

pub struct DSU {
    parent: Vec<Cell<u32>>,
    size: Vec<u32>,
}

impl DSU {
    pub fn new(size: usize) -> DSU {
        DSU {
            parent: (0..size as u32).map(Cell::new).collect(),
            size: vec![1; size],
        }
    }

    pub fn get(&self, u: u32) -> u32 {
        let mut u = u;
        while u != self.parent[u as usize].get() {
            let p = self.parent[u as usize].get();
            let g = self.parent[p as usize].get();
            self.parent[u as usize].set(g);
            u = g;
        }
        u
    }

    pub fn size(&self, u: u32) -> u32 {
        self.size[self.get(u) as usize]
    }

    pub fn is_same(&self, u: u32, v: u32) -> bool {
        self.get(u) == self.get(v)
    }

    pub fn merge(&mut self, u: u32, v: u32) -> bool {
        let mut u = self.get(u);
        let mut v = self.get(v);
        if u == v {
            return false;
        }

        if self.size[u as usize] < self.size[v as usize] {
            (u, v) = (v, u);
        }

        self.parent[v as usize].set(u);
        self.size[u as usize] += self.size[v as usize];

        true
    }
}
