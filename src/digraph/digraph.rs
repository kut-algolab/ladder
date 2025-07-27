use std::collections::{HashSet, HashMap, VecDeque};

type Vertex = String;
type Arc = (usize, usize); // １つ目が弧 (u, v) の終点 v，2つ目は弧の重みを意味

/// グラフ構造体
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Digraph {
    V: HashSet<Vertex>,             // 頂点集合
    A: Vec<HashSet<Arc>>,           // 弧集合
    number: HashMap<Vertex, usize>, // 単語から頂点番号へのマップ
    label: Vec<Vertex>,             // 頂点番号から単語へのマップ
}

impl Digraph {
    pub fn read_vertices_and_make_graph(vs: Vec<String>) -> Digraph {
        let mut vertices = HashSet::new();
        let mut number_map = HashMap::new();
        let mut label_vec = Vec::new();
	
	let n = vs.len();
        for (i, v) in vs.iter().enumerate() {
            vertices.insert(v.clone());
            number_map.insert(v.clone(), i);
            label_vec.push(v.clone());
        }

        // 隣接リスト
        let mut adj: Vec<HashSet<Arc>> = vec![HashSet::new(); n];
        
        // 単語の開始文字をキーとして，単語リストへのマップを作成
        let mut start_char_map: HashMap<char, Vec<&Vertex>> = HashMap::new();
	for word in &vertices {
	    if let Some(first_char) = word.chars().next() {
		start_char_map.entry(first_char).or_default().push(word);
	    }
	}

	for (u_idx, u_word) in label_vec.iter().enumerate() {
	    if let Some(end_char) = u_word.chars().last() {
		if let Some(vec) = start_char_map.get(&end_char) {
		    for v_word in vec {
			if let Some(&v_idx) = number_map.get(*v_word) {
			    if u_idx != v_idx {
				adj[u_idx].insert((v_idx, 1));
			    }
			}
		    }
		}
	    }
	}
	
        Digraph {
            V: vertices,
            A: adj,
            number: number_map,
            label: label_vec,
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
            for (u, _) in &self.A[v] {
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
}
