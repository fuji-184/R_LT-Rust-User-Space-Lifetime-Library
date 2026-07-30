use lifetime_cli::*;
use std::collections::HashSet;

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
    let mut seen = HashSet::<String>::new();
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

fn print_usage() {
    eprintln!("Usage: cargo lifetime check [--file <path>]");
    eprintln!("       cargo lifetime check <path>");
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
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}
