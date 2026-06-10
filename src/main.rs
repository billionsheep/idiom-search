use rusqlite::{Connection, Result, params};
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct Idiom {
    word: String,
    pinyin: String,
    meaning: String,
    origin: String,
    story: String,
    example: String,
    rare_level: String,
    tags: String,
}

enum Command {
    Search { keyword: String, limit: usize },
    Exact { word: String },
    ListRare { limit: usize },
    Stats,
    Help,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("错误: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let command = parse_args(env::args().skip(1).collect());

    if matches!(command, Command::Help) {
        print_help();
        return Ok(());
    }

    let conn = Connection::open(default_db_path())?;

    match command {
        Command::Search { keyword, limit } => {
            let idioms = search_idioms(&conn, &keyword, limit)?;
            print_results(&idioms);
        }
        Command::Exact { word } => {
            let idioms = exact_idiom(&conn, &word)?;
            print_results(&idioms);
        }
        Command::ListRare { limit } => {
            let idioms = list_rare_idioms(&conn, limit)?;
            print_results(&idioms);
        }
        Command::Stats => print_stats(&conn)?,
        Command::Help => unreachable!(),
    }

    Ok(())
}

fn default_db_path() -> PathBuf {
    env::var_os("IDIOM_DB_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data/idioms.sqlite"))
}

fn parse_args(args: Vec<String>) -> Command {
    if args.is_empty() {
        return Command::Help;
    }

    match args[0].as_str() {
        "search" => {
            let keyword = args.get(1).cloned().unwrap_or_default();
            if keyword.is_empty() {
                return Command::Help;
            }
            Command::Search {
                keyword,
                limit: parse_limit(&args, 20),
            }
        }
        "exact" => {
            let word = args.get(1).cloned().unwrap_or_default();
            if word.is_empty() {
                return Command::Help;
            }
            Command::Exact { word }
        }
        "list" if args.iter().any(|arg| arg == "--rare") => Command::ListRare {
            limit: parse_limit(&args, 20),
        },
        "stats" => Command::Stats,
        "help" | "-h" | "--help" => Command::Help,
        _ => Command::Search {
            keyword: args.join(" "),
            limit: parse_limit(&args, 20),
        },
    }
}

fn parse_limit(args: &[String], default_limit: usize) -> usize {
    args.windows(2)
        .find(|window| window[0] == "--limit" || window[0] == "-n")
        .and_then(|window| window[1].parse::<usize>().ok())
        .filter(|limit| *limit > 0)
        .unwrap_or(default_limit)
}

fn search_idioms(conn: &Connection, keyword: &str, limit: usize) -> Result<Vec<Idiom>> {
    let pattern = format!("%{keyword}%");
    let mut stmt = conn.prepare(
        "
        SELECT word, pinyin, meaning, origin, story, example, rare_level, tags
        FROM idioms
        WHERE word LIKE ?1
           OR pinyin LIKE ?1
           OR meaning LIKE ?1
           OR origin LIKE ?1
           OR story LIKE ?1
           OR example LIKE ?1
           OR tags LIKE ?1
        ORDER BY
            CASE WHEN word = ?2 THEN 0 WHEN word LIKE ?1 THEN 1 ELSE 2 END,
            word ASC
        LIMIT ?3
        ",
    )?;
    collect_idioms(&mut stmt, params![pattern, keyword, limit as i64])
}

fn exact_idiom(conn: &Connection, word: &str) -> Result<Vec<Idiom>> {
    let mut stmt = conn.prepare(
        "
        SELECT word, pinyin, meaning, origin, story, example, rare_level, tags
        FROM idioms
        WHERE word = ?1
        ORDER BY word ASC
        ",
    )?;
    collect_idioms(&mut stmt, params![word])
}

fn list_rare_idioms(conn: &Connection, limit: usize) -> Result<Vec<Idiom>> {
    let mut stmt = conn.prepare(
        "
        SELECT word, pinyin, meaning, origin, story, example, rare_level, tags
        FROM idioms
        WHERE rare_level IN ('较少见', '冷僻')
        ORDER BY CASE rare_level WHEN '冷僻' THEN 0 ELSE 1 END, word ASC
        LIMIT ?1
        ",
    )?;
    collect_idioms(&mut stmt, params![limit as i64])
}

fn collect_idioms<P>(stmt: &mut rusqlite::Statement<'_>, params: P) -> Result<Vec<Idiom>>
where
    P: rusqlite::Params,
{
    let rows = stmt.query_map(params, |row| {
        Ok(Idiom {
            word: row.get(0)?,
            pinyin: row.get(1)?,
            meaning: row.get(2)?,
            origin: row.get(3)?,
            story: row.get(4)?,
            example: row.get(5)?,
            rare_level: row.get(6)?,
            tags: row.get(7)?,
        })
    })?;

    rows.collect()
}

fn print_results(idioms: &[Idiom]) {
    if idioms.is_empty() {
        println!("未找到匹配的成语。");
        return;
    }

    for (index, idiom) in idioms.iter().enumerate() {
        println!("{}. {}", index + 1, idiom.word);
        println!("   读音: {}", idiom.pinyin);
        println!("   释义: {}", idiom.meaning);
        println!("   出处: {}", idiom.origin);
        println!("   典故: {}", idiom.story);
        println!("   例句: {}", idiom.example);
        println!("   冷僻程度: {}", idiom.rare_level);
        println!("   标签: {}", idiom.tags);
        if index + 1 < idioms.len() {
            println!();
        }
    }
}

fn print_stats(conn: &Connection) -> Result<()> {
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM idioms", [], |row| row.get(0))?;
    let rare: i64 = conn.query_row(
        "SELECT COUNT(*) FROM idioms WHERE rare_level IN ('较少见', '冷僻')",
        [],
        |row| row.get(0),
    )?;

    println!("成语总数: {total}");
    println!("较少见/冷僻: {rare}");
    println!("数据文件: {}", default_db_path().display());
    Ok(())
}

fn print_help() {
    println!(
        "\
idiom-search - 轻量化成语检索工具

用法:
  cargo run -- search <关键词> [--limit 20]  按成语、拼音、释义、典故等模糊查询
  cargo run -- exact <成语>                  精确查询一个成语
  cargo run -- list --rare [--limit 20]      列出较少见/冷僻成语
  cargo run -- stats                         查看数据统计

示例:
  cargo run -- exact 画蛇添足
  cargo run -- search 多此一举
  cargo run -- search 战国 --limit 5
  cargo run -- list --rare

环境变量:
  IDIOM_DB_PATH=自定义 SQLite 数据文件路径
"
    );
}
