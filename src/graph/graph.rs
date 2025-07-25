use petgraph::unionfind::UnionFind;
use std::collections::{HashMap, HashSet, VecDeque};

type Vertex = String;
type Edge = (usize, usize); // ((u, v) の端点 v, 重み)

/// グラフ構造体
/// 単語を頂点とし，差分が1の単語へ有向辺をはる
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Graph {
    V: HashSet<Vertex>,               // 頂点集合
    E: Vec<HashSet<Edge>>,            // 辺集合
    number: HashMap<Vertex, usize>,   // 単語から頂点番号へのマップ
    label: Vec<Vertex>,               // 頂点番号から単語へのマップ
}

impl Graph {
    /// 高速なグラフ構築
    pub fn read_vertices_and_make_graph(vs: Vec<String>) -> Graph {
        let mut i = 0;
        let mut vertices = HashSet::new();
        let mut number_map = HashMap::new();
        let mut label_vec = Vec::new();

        for v in vs {
            if vertices.insert(v.clone()) {
                number_map.insert(v.clone(), i);
                label_vec.push(v);
                i += 1;
            }
        }

	// 隣接リスト
        let n = vertices.len();
        let mut adj: Vec<HashSet<Edge>> = vec![HashSet::new(); n];

	// 単語の長さ単位でマップへ追加
        let mut words_by_len: HashMap<usize, Vec<&Vertex>> = HashMap::new();
        for word in &vertices {
            words_by_len.entry(word.chars().count()).or_default().push(word);
        }

	// 長さ単位で処理
        for (_, words) in &words_by_len {
	    // 長さが2未満の単語はまとめて処理
            if words.len() < 2 { continue; }
            let word_len = words[0].chars().count();
            let mut wildcard_map: HashMap<String, Vec<&Vertex>> = HashMap::new();

	    // パターン(word -> *ord, w*rd, ...) をキーとして一致する単語リストを構築
            for word in words {
                let mut chars: Vec<char> = word.chars().collect();
                for i in 0..word_len {
                    let original_char = chars[i];
                    chars[i] = '*';
                    let pattern: String = chars.iter().collect();
                    wildcard_map.entry(pattern).or_default().push(word);
                    chars[i] = original_char;
                }
            }

	    // 有向辺の追加
            for group in wildcard_map.values() {
		// 要素数が2以上なら隣接する単語が存在
                if group.len() < 2 { continue; }
                for i in 0..group.len() {
                    for j in i + 1..group.len() {
                        let u_idx = *number_map.get(group[i]).unwrap();
                        let v_idx = *number_map.get(group[j]).unwrap();
                        adj[u_idx].insert((v_idx, 1));
                        adj[v_idx].insert((u_idx, 1));
                    }
                }
            }
        }

	// 長さのみの Vec を構築・昇順にソート
        let mut sorted_lens: Vec<_> = words_by_len.keys().cloned().collect();
        sorted_lens.sort_unstable();

	// 隣り合う長さの文字列について処理
        for i in 0..sorted_lens.len().saturating_sub(1) {
            let len1 = sorted_lens[i];
            let len2 = sorted_lens[i + 1];

	    // 長さが2以上異なる場合はスキップ
            if len1 + 1 != len2 { continue; }

	    // 短い方の単語リストを HashSet, 長い方の単語リストを Vec で構築
            let shorter_words_set: HashSet<&str> = words_by_len.get(&len1).unwrap().iter().map(|v| v.as_str()).collect();
            let longer_words: &Vec<&Vertex> = words_by_len.get(&len2).unwrap();

	    // 長い方の単語から一文字削除した単語が，短い方の単語リストに含まれていれば有向辺を追加
            for &long_word in longer_words {
                let long_chars: Vec<char> = long_word.chars().collect();
                for j in 0..long_chars.len() {
                    let mut candidate_chars = long_chars.clone();
                    candidate_chars.remove(j);
                    let candidate: String = candidate_chars.iter().collect();

                    if let Some(short_word_str) = shorter_words_set.get(candidate.as_str()) {
                        let u_idx = *number_map.get(*short_word_str).unwrap();
                        let v_idx = *number_map.get(long_word).unwrap();
                        adj[u_idx].insert((v_idx, 1));
                        adj[v_idx].insert((u_idx, 1));
                    }
                }
            }
        }

        Graph {
            V: vertices,
            E: adj,
            number: number_map,
            label: label_vec,
        }
    }
    pub fn print_graph(&self) {
	for u in 0..self.V.len() {
	    for (v, _) in &self.E[u] {
		println!("{} -- {}", self.label[u], self.label[*v]);
	    }
	}
    }

    /// 単語ラベルを指定して最短経路を探索
    pub fn path_by_label(&self, s: &String, t: &String) -> Option<Vec<Vertex>> {
        let start_node = self.number.get(s);
        let end_node = self.number.get(t);

	// ラベル s が存在しない
        if start_node.is_none() {
            println!("{} は存在しません．", s);
            return None;
        }
	// ラベル t が存在しない
        if end_node.is_none() {
            println!("{} は存在しません．", t);
            return None;
        }
        
        if let Some(path_indices) = self.path(*start_node.unwrap(), *end_node.unwrap()) {
            let path_labels = path_indices.iter().map(|&v| self.label[v].clone()).collect();
            Some(path_labels)
        } else {
	    None
        }
    }

    /// s -- t の最短経路を BFS で探索
    fn path(&self, s: usize, t: usize) -> Option<Vec<usize>> {
        if s == t { return Some(vec![s]); }

	// 直前の単語を格納 (訪問済みの確認にも利用)
        let mut prev: Vec<Option<usize>> = vec![None; self.V.len()];

        let mut queue = VecDeque::new();
        queue.push_back(s);
        prev[s] = Some(s);

	// Deque による BFS
        'bfs_loop: while let Some(v) = queue.pop_front() {
            for (u, _) in &self.E[v] {
		// u が未探索の場合 Deque の末尾に u を追加
                if prev[*u].is_none() {
                    prev[*u] = Some(v);
                    queue.push_back(*u);
		    // 到達判定
                    if *u == t {
                        break 'bfs_loop;
                    }
                }
            }
        }

	// 経路の復元
        if prev[t].is_some() {
            let mut path = Vec::new();
            let mut curr = t;
            while curr != s {
                path.push(curr);
                curr = prev[curr].unwrap();
            }
            path.push(s);
            path.reverse();
            Some(path)
        } else {
            None
        }
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
