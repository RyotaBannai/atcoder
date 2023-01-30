#[derive(Clone, Debug, PartialEq)]
struct Node {
    pub next: Vec<isize>,   // 子の頂点番号(文字列なら、文字)
    pub accept: Vec<usize>, // 末端がこの頂点になる文字列の`id`
    pub c: usize,           // base からの間隔を usize で表現. a~z なら base は A
    pub common: usize,      // いくつの文字列がこの頂点を共有しているか
}
impl Node {
    // 前提: c はbase を差し引いている
    fn new(c: usize) -> Self {
        Self {
            next: vec![-1; 26], // 0 index
            accept: vec![],
            c,
            common: 0, // 頂点を作った時点文字列の末端かどうかはわからないから 0
        }
    }
}

use std::usize::MAX;
#[derive(Clone, Debug, PartialEq)]
pub struct Trie {
    nodes: Vec<Node>,
    root: usize, // 根
    base: char,
}
impl Trie {
    pub fn new(base: char) -> Self {
        Self {
            root: 0,
            nodes: vec![Node::new(MAX)],
            base,
        }
    }
    pub fn insert(&mut self, item: &[char], item_id: usize) {
        let mut node_id = self.root;
        for &x in item {
            let c = self.alp_to_i(x);
            let mut next_id = self.nodes[node_id].next[c];
            if next_id == -1 {
                next_id = self.nodes.len() as isize;
                self.nodes.push(Node::new(c)); // 同じ頂点の子がまとまって管理されているわけではない.
                self.nodes[node_id].next[c] = next_id;
            }
            self.nodes[node_id].common += 1;
            node_id = next_id as usize;
        }
        self.nodes[node_id].common += 1;
        self.nodes[node_id].accept.push(item_id);
    }
    // Trie からitem で与えた文字列と一致する文字列が存在するか
    pub fn search(&self, item: &[char]) -> bool {
        let mut node_id = self.root;
        for &x in item {
            let c = self.alp_to_i(x);
            let next_id = self.nodes[node_id].next[c];
            if next_id == -1 {
                return false;
            }
            node_id = next_id as usize;
        }
        !self.nodes[node_id].accept.is_empty()
    }
    // Trie からitem で与えた文字列と一番長く一致する文字列を返す.
    // s-t に対して、teim=str であればst
    // great common sub string
    pub fn find_gcss(&self, item: &[char]) -> Vec<char> {
        let mut node_id = self.root;
        let mut s: Vec<char> = vec![];
        for &x in item {
            let c = self.alp_to_i(x);
            let next_id = self.nodes[node_id].next[c];
            if next_id == -1 || self.nodes[next_id as usize].common < 2 {
                // commmon <2 自分以外同じ部分文字列が文字列が存在しない.
                // 全く同じ文字列が２回挿入された場合は common >=2
                return s;
            }
            s.push(x);
            node_id = next_id as usize;
        }
        s
    }
    fn alp_to_i(&self, c: char) -> usize {
        (c as u8 - self.base as u8) as usize
    }
}
