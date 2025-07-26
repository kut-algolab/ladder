use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
// use std::time::Instant;
use ladder::graph::Graph;

fn main() {
    // プログラムの引数
    // 引数は，[1] 辞書ファイルのパス，[2] 単語長の下限，[3] 単語長の上限
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
	eprintln!("Usage: {} <filepath> <min_len> <max_len>", args[0]);
	return;
    }

    // ファイルのパス名
    let filename: String = args[1].parse().expect("このファイルは読めません．");

    // l 文字以上 u 文字以下の単語を使用
    let l: usize = args[2]
        .parse()
        .expect("単語長の下限の読み込みに失敗しました");
    let u: usize = args[3]
        .parse()
        .expect("単語長の上限の読み込みに失敗しました");

    // ファイルオープン
    let mut file = match File::open(&filename) {
        Err(s) => panic!("ファイル {} を開けません: {}", filename, s),
        Ok(file) => file,
    };

    // ファイル読込
    let mut lines = String::new();
    match file.read_to_string(&mut lines) {
        Err(s) => panic!("ファイル {} を読めません: {}", filename, s),
        Ok(_) => (),
    };

    // ファイルの中身を words へ追加
    let mut words = Vec::new();
    for line in lines.split_whitespace() {
        words.push(line);
    }

    // u 以上 l 以下の文字のみ残すようフィルタ
    let mut filtered_words = Vec::new();
    for w in words {
        if 3 * l <= w.len() && w.len() <= 3 * u {
            filtered_words.push(w.to_string());
        }
    }

    // 構築時間を計測
    // let start = Instant::now();

    // 単語を頂点とし，一文字の置換・追加・削除で移り変われる単語間に辺を持つグラフを作成
    let g = Graph::read_vertices_and_make_graph(filtered_words);

    // let duration = start.elapsed();
    // println!("構築に{:?}秒かかりました．", duration);
    // g.print_isolated_vertices();

    loop {
        print!("始点となる単語を入力してください: ");
        io::stdout().flush().unwrap();

        let mut start = String::new();
        io::stdin()
            .read_line(&mut start)
            .expect("Failed to read line");
        let start = start.trim().to_string();

        if start == "quit" || start == "exit" {
            break;
        }

        print!("終点となる単語を入力してください: ");
        io::stdout().flush().unwrap();
        let mut end = String::new();
        io::stdin()
            .read_line(&mut end)
            .expect("Failed to read line");
        let end = end.trim().to_string();

        let p = g.path_by_label(&start, &end);
        match p {
            None => println!("{} から {} への経路は存在しません．", start, end),
            Some(path) => {
                print!("{}", path[0]);
                for i in 1..path.len() {
                    print!(" -> {}", path[i]);
                }
                println!()
            }
        }
    }
}
