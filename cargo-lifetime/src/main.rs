include!("analysis.rs");

const LINE_OFFSET: usize = 1;

const CYAN: &str = "\x1b[1;36m";
const YELLOW: &str = "\x1b[1;33m";
const RED: &str = "\x1b[1;31m";
const GREEN: &str = "\x1b[1;32m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

fn colorize(msg: &str) -> String {
    let mut r = String::new();
    let mut i = 0;
    let b = msg.as_bytes();
    while i < b.len() {
        if b[i] == b'[' {
            if let Some(end) = msg[i..].find(']') {
                r.push_str(YELLOW);
                r.push_str(&msg[i..=i + end]);
                r.push_str(RESET);
                i += end + 1;
                continue;
            }
        }
        if b[i] == b'`' {
            if let Some(end) = msg[i + 1..].find('`') {
                r.push('`');
                r.push_str(CYAN);
                r.push_str(&msg[i + 1..=i + end]);
                r.push_str(RESET);
                r.push('`');
                i += end + 2;
                continue;
            }
        }
        if msg[i..].starts_with("(declared at line ") {
            if let Some(end) = msg[i..].find(')') {
                r.push_str(DIM);
                r.push_str(&msg[i..=i + end]);
                r.push_str(RESET);
                i += end + 1;
                continue;
            }
        }
        r.push(msg[i..].chars().next().unwrap());
        i += msg[i..].chars().next().unwrap().len_utf8();
    }
    r
}

fn pline(path: &str, line: usize, msg: &str) {
    println!("{CYAN}{path}:{line}:{RESET} {}", colorize(msg));
}

fn adjust_line_in_msg(msg: &str, n: usize) -> String {
    let mut r = String::new();
    let mut rest = msg;
    while let Some(start) = rest.find("(declared at line ") {
        r.push_str(&rest[..start]);
        r.push_str("(declared at line ");
        let off = start + "(declared at line ".len();
        let end = rest[off..].find(')').map(|e| off + e).unwrap_or(rest.len());
        let raw: usize = rest[off..end].parse().unwrap_or(1);
        r.push_str(&raw.saturating_sub(n).to_string());
        r.push(')');
        rest = &rest[end + 1..];
    }
    r.push_str(rest);
    r
}

fn check_file(path: &str) -> Vec<(usize, String)> {
    let src = std::fs::read_to_string(path).unwrap();
    check_source(&src)
}

fn find_module_files(src: &str, base_dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for line in src.lines() {
        let s = line.trim();
        if !s.starts_with("mod ") {
            continue;
        }
        let rest = &s[4..];
        let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        if name.is_empty() || !is_ident(&name) {
            continue;
        }
        let rest = &rest[name.len()..].trim();
        if !rest.starts_with(';') {
            continue;
        }
        let candidates = [
            format!("{}/{}.rs", base_dir, name),
            format!("{}/{}/mod.rs", base_dir, name),
        ];
        for c in &candidates {
            if std::path::Path::new(c).exists() {
                files.push(c.clone());
                break;
            }
        }
    }
    files
}

fn check_project() -> Vec<(String, usize, String)> {
    let entry = if std::path::Path::new("src/main.rs").exists() {
        "src/main.rs".to_string()
    } else if std::path::Path::new("src/lib.rs").exists() {
        "src/lib.rs".to_string()
    } else {
        return Vec::new();
    };

    let mut all = Vec::new();
    let mut seen = std::collections::HashSet::<String>::new();
    let mut stack = vec![entry];

    while let Some(path) = stack.pop() {
        if !seen.insert(path.clone()) {
            continue;
        }
        let src = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let dir = std::path::Path::new(&path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        for mod_file in find_module_files(&src, &dir) {
            stack.push(mod_file);
        }
        for (line, msg) in check_source(&src) {
            all.push((path.clone(), line, msg));
        }
    }
    all
}

fn run_tests() {
    let test_dir = format!("{}/../tests", env!("CARGO_MANIFEST_DIR"));
    let mut cases: Vec<_> = std::fs::read_dir(&test_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|s| s == "rs").unwrap_or(false))
        .collect();
    cases.sort_by_key(|e| e.file_name());

    let results: Vec<(String, Vec<(usize, String)>, bool)> = std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(cases.len());

        for entry in &cases {
            let name = entry.file_name().to_string_lossy().to_string();
            let body = std::fs::read_to_string(entry.path()).unwrap();
            let body_line_count = body.lines().count();
            let expect_err = name.starts_with("invalid_");

            handles.push(s.spawn(move || {
                let src = format!(
                    r#"macro_rules! lt {{ ($e:expr, $l:expr) => {{ $e }}; }}
fn main() {{ {} }}"#,
                    body
                );
                let errors = check_source(&src);
                let errors: Vec<(usize, String)> = errors
                    .into_iter()
                    .map(|(line, msg)| {
                        let orig = if line > LINE_OFFSET {
                            (line - LINE_OFFSET).min(body_line_count)
                        } else {
                            1
                        };
                        let msg = adjust_line_in_msg(&msg, LINE_OFFSET);
                        (orig, msg)
                    })
                    .collect();
                let ok = if expect_err { !errors.is_empty() } else { errors.is_empty() };
                (name, errors, ok)
            }));
        }

        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    let mut pass = 0u32;
    let mut fail = 0u32;

    for (name, errors, ok) in &results {
        if *ok {
            println!("  {GREEN}PASS{RESET}  tests/{name}");
            pass += 1;
        } else {
            println!("  {RED}FAIL{RESET}  tests/{name}");
            fail += 1;
        }
        for (line, msg) in errors {
            print!("        ");
            pline(name, *line, msg);
        }
    }

    let total = pass + fail;
    let n_cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    if fail > 0 {
        println!("---\n{GREEN}{pass}{RESET}/{total} passed, {RED}{fail}{RESET} failed  [{n_cores} cores]");
        std::process::exit(1);
    } else {
        println!("---\n{GREEN}{pass}{RESET}/{total} passed, {RED}{fail}{RESET} failed  [{n_cores} cores]");
    }
}

fn print_file_errors(path: &str, errors: &[(usize, String)]) {
    for (line, msg) in errors {
        pline(path, *line, msg);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("check") => {
            match args.get(2).map(|s| s.as_str()) {
                Some("--file") => {
                    let path = args.get(3).map(|s| s.as_str()).unwrap_or("src/main.rs");
                    let errors = check_file(path);
                    print_file_errors(path, &errors);
                    if !errors.is_empty() {
                        std::process::exit(1);
                    }
                }
                Some(p) if !p.starts_with('-') => {
                    let errors = check_file(p);
                    print_file_errors(p, &errors);
                    if !errors.is_empty() {
                        std::process::exit(1);
                    }
                }
                _ => {
                    let errors = check_project();
                    if errors.is_empty() {
                        return;
                    }
                    for (path, line, msg) in &errors {
                        pline(path, *line, msg);
                    }
                    std::process::exit(1);
                }
            }
        }
        _ => run_tests(),
    }
}
