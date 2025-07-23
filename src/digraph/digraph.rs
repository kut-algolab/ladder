use std::collections::HashSet;

type Vertex = String;
type Arc = (usize, usize); // １つ目が弧 (u, v) の終点 v，2つ目は弧の重みを意味

#[derive(Debug)]
pub struct Digraph {
    V: HashSet<Vertex>,   // 頂点集合
    A: Vec<HashSet<Arc>>, // 弧集合
}
