use petgraph::unionfind::UnionFind;
use std::collections::{HashMap, HashSet, VecDeque};

const INF: usize = 1 << 30;

type Vertex = String;
type Edge = (usize, usize); // １つ目が辺 (u, v) の端点 v，2つ目は辺の重みを意味

pub fn is_adjacent(u: &Vertex, v: &Vertex) -> bool {
    // ユニコードは一文字を3バイトで表現するため，長さ二文字異なるということは 4 だけ違う
    if 4 <= (u.len() as isize - v.len() as isize).abs() {
        return false;
    }
    let u: Vec<char> = u.chars().collect();
    let v: Vec<char> = v.chars().collect();

    // 長さが同じ場合
    if u.len() == v.len() {
        let mut diff = 0;
        for i in 0..u.len() {
            if u[i] != v[i] {
                diff += 1;
            }
        }
        if diff == 1 {
            return true;
        } else {
            return false;
        }
    }
    // 長さが 1 違う場合（小谷さんのアルゴリズムをリスペクト）
    let mut x = &u;
    let mut y = &v;
    if v.len() < u.len() {
        y = &u;
        x = &v;
    }
    let mut diff = 0;
    let mut i = 0;
    let mut j = 0;

    while i < x.len() && j < y.len() {
        if x[i] == y[j] {
            i += 1;
            j += 1;
        } else {
            if diff == 1 {
                return false;
            }
            diff += 1;
            j += 1;
        }
    }
    true
}

pub fn is_adjacent2(u: &Vec<char>, v: &Vec<char>) -> bool {
    if 2 <= (u.len() as isize - v.len() as isize).abs() {
        return false;
    }

    // 長さが同じ場合
    if u.len() == v.len() {
        let mut diff = 0;
        for i in 0..u.len() {
            if u[i] != v[i] {
                diff += 1;
            }
        }
        if diff == 1 {
            return true;
        } else {
            return false;
        }
    }
    // 長さが 1 違う場合（小谷さんのアルゴリズムをリスペクト）
    let mut x = &u;
    let mut y = &v;
    if v.len() < u.len() {
        y = &u;
        x = &v;
    }
    let mut diff = 0;
    let mut i = 0;
    let mut j = 0;

    while i < x.len() && j < y.len() {
        if x[i] == y[j] {
            i += 1;
            j += 1;
        } else {
            if diff == 1 {
                return false;
            }
            diff += 1;
            j += 1;
        }
    }
    true
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Graph {
    V: HashSet<Vertex>,             // 頂点集合
    E: Vec<HashSet<Edge>>,          // 辺集合
    number: HashMap<Vertex, usize>, // 頂点ラベルを頂点番号返すためのマップ
    label: Vec<Vertex>, // 頂点番号から頂点ラベルを返すためのベクタ（0から始まるため HashMap ではなくベクタを使用）
}

impl Graph {
    pub fn read_vertices_and_make_graph(vs: Vec<String>) -> Graph {
        let mut i = 0; // 頂点番号
        let mut xs = HashSet::new(); // 頂点集合
        let mut es: Vec<HashSet<Edge>> = vec![HashSet::new(); vs.len()]; // 辺集合（アルゴリズム用）
        let mut _number = HashMap::new(); // 頂点のラベルから頂点番号を返すマップ
        let mut _label = Vec::new(); // 頂点番号から頂点ラベルを返すためのマップ
        for v in &vs {
            if xs.insert(v.clone()) {
                _number.insert(v.clone(), i);
                _label.push(v.clone());
                i += 1;
            }
        }
        let mut ws = Vec::new();
        for v in &vs {
            ws.push(v.chars().collect());
        }
        for i in 0..ws.len() - 1 {
            for j in i + 1..ws.len() {
                if is_adjacent2(&ws[i], &ws[j]) {
                    es[i].insert((j, 1));
                    es[j].insert((i, 1));
                }
            }
        }
        Graph {
            V: xs,
            E: es,
            number: _number,
            label: _label,
        }
    }
    pub fn print_graph(&self) {
        for u in 0..self.V.len() {
            for (v, _) in &self.E[u] {
                println!("{} -- {}", self.label[u], self.label[*v]);
            }
        }
    }
}

// グラフ探索アルゴリズム
impl Graph {
    // s - t パスを一つ返す関数
    pub fn path_by_label(&self, s: &String, t: &String) -> Option<Vec<Vertex>> {
        let ns = self.number.get(s);
        let nt = self.number.get(t);

        // s もしくは t というラベルの頂点は存在しない
        if ns == None {
            println!("{} は存在しません．", s);
            return None;
        } else if nt == None {
            println!("{} は存在しません．", t);
            return None;
        }

        let p = self.path(*ns.unwrap(), *nt.unwrap());
        if p == None {
            return None;
        }

        let p = p.unwrap();
        let x: Vec<Vertex> = p.iter().map(|&v| self.label[*&v].clone()).collect();
        Some(x)
    }

    // s - t パスを一つ返す（最短経路）
    pub fn path(&self, s: usize, t: usize) -> Option<Vec<usize>> {
        let mut p = vec![]; // s から t への経路
        let mut prev = vec![INF; self.V.len()]; // s -> t の経路について各頂点の一つ前の頂点を格納

        prev[s] = INF + 1; // prev[v] == INF は v が未探索を意味するため，INF 以外の影響しない値を代入
        let mut que = VecDeque::new(); // 幅優先探索のための Deque を準備
        que.push_back(s);
        while !que.is_empty() {
            // Deque の先頭要素を取り出す
            let v = que.pop_front().unwrap();
            for (u, _) in &self.E[v] {
                // u が未探索の場合は Deque の末尾に u を追加
                if prev[*u] == INF {
                    prev[*u] = v; // u の一つ前の頂点を v に設定
                    que.push_back(*u);
                }
            }
        }

        // t から初めて prev をたどり，s に到達するまで遡る
        let mut v = t;
        while v != s && v != INF {
            p.push(v);
            v = prev[v];
        }

        // 始点 s まで辿れなかった場合は s -> t パスは存在しない
        if v != s {
            return None;
        }

        p.push(s);
        p.reverse(); // 逆順に頂点を追加したため順番を反転させる
        Some(p)
    }

    fn dfs(&self, v: usize, visited: &mut Vec<bool>, uf: &mut UnionFind<usize>) {
        visited[v] = true;
        for (u, _) in &self.E[v] {
            if !visited[*u] {
                uf.union(v, *u);
                self.dfs(*u, visited, uf);
            }
        }
    }

    pub fn connected_components(&self) -> UnionFind<usize> {
        let mut uf = UnionFind::new(self.V.len());
        let mut visited = vec![false; self.V.len()];
        self.dfs(0, &mut visited, &mut uf);
        uf
    }
}

// 簡単な関数
impl Graph {
    // 次数 0 の頂点（孤立点）を出力
    pub fn print_isolated_vertices(&self) {
        let mut num = 0;
        for i in 0..self.label.len() {
            if self.E[i].len() == 0 {
                println!("{}", self.label[i]);
                num += 1;
            }
        }
        println!("他の単語と隣接しない単語は {} あります．", num);
    }
}
