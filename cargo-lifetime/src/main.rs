use lifetime_cli::*;
use std::collections::HashSet;

fn check_file(path: &str) -> Result<Vec<(usize, String)>, String> {
    let src = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => return Err(format!("error reading `{}`: {}", path, e)),
    };
    Ok(check_source(&src))
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

fn find_inline_modules(src: &str) -> Vec<(String, String)> {
    let mut modules = Vec::new();
    let mut i = 0;
    let lines: Vec<&str> = src.lines().collect();
    while i < lines.len() {
        let s = lines[i].trim();
        if s.starts_with("mod ") && !s.ends_with(';') {
            let rest = &s[4..];
            let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            if !name.is_empty() && is_ident(&name) {
                let after_name = &rest[name.len()..].trim();
                if after_name.starts_with('{') {
                    let mut depth: isize = 0;
                    let start = i;
                    let mut end = i;
                    for (j, line) in lines[i..].iter().enumerate() {
                        for ch in line.chars() {
                            match ch {
                                '{' => depth += 1,
                                '}' => {
                                    depth -= 1;
                                    if depth == 0 {
                                        end = i + j;
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                        if depth == 0 && end != i {
                            break;
                        }
                    }
                    if end > start {
                        let body: String = lines[start + 1..=end - 1].iter().map(|l| format!("{}\n", l)).collect();
                        modules.push((name, body));
                        i = end;
                    }
                }
            }
        }
        i += 1;
    }
    modules
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
        for (name, body) in find_inline_modules(&src) {
            let inline_path = format!("{} (inline mod {})", path, name);
            if seen.insert(inline_path.clone()) {
                for (line, msg) in check_source(&body) {
                    all.push((format!("{}:{}", path, name), line, msg));
                }
            }
        }
        for (line, msg) in check_source(&src) {
            all.push((path.clone(), line, msg));
        }
    }
    all
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  cargo lifetime check                Auto-detect & traverse modules");
    eprintln!("  cargo lifetime check --file <path>  Check a single file");
    eprintln!("  cargo lifetime check <path>         Check a specific file");
    eprintln!("  cargo lifetime --help               Show this help");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--help") | Some("-h") => {
            print_usage();
            return;
        }
        Some("check") => {
            match args.get(2).map(|s| s.as_str()) {
                Some("--help") | Some("-h") => {
                    eprintln!("Usage: cargo lifetime check [--file <path> | <path>]");
                    return;
                }
                Some("--file") => {
                    let path = match args.get(3) {
                        Some(p) => p.as_str(),
                        None => {
                            eprintln!("error: --file requires a path argument");
                            eprintln!("Usage: cargo lifetime check --file <path>");
                            std::process::exit(1);
                        }
                    };
                    match check_file(path) {
                        Ok(errors) => {
                            print_file_errors(path, &errors);
                            if !errors.is_empty() {
                                std::process::exit(1);
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Some(p) if !p.starts_with('-') => {
                    match check_file(p) {
                        Ok(errors) => {
                            print_file_errors(p, &errors);
                            if !errors.is_empty() {
                                std::process::exit(1);
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
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
