use std::{
    collections::HashMap,
    path::Path,
    env,
    fs,
    io,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = get_file_path(&args)?;
    let content = fs::read_to_string(file_path)?;
    let organized = organize_rust_imports(&content);
    fs::write(file_path, organized)?;
    println!("Imports organized successfully.");
    Ok(())
}

fn get_file_path(args: &[String]) -> io::Result<&Path> {
    args.get(1)
        .map(Path::new)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Usage: imporg <file>"))
}

fn organize_rust_imports(code: &str) -> String {
    let mut std_lib = Vec::new();
    let mut external = Vec::new();
    let mut internal = Vec::new();
    let mut non_import_lines = Vec::new();
    let mut in_import_section = false;

    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("use ") {
            in_import_section = true;
            if trimmed.starts_with("use std::") {
                std_lib.push(trimmed);
            } else if trimmed.starts_with("use crate::") {
                internal.push(trimmed);
            } else {
                external.push(trimmed);
            }
        } else if in_import_section && !trimmed.is_empty() {
            in_import_section = false;
        }
        if !in_import_section {
            non_import_lines.push(line);
        }
    }

    fn group_imports(imports: &[&str]) -> HashMap<String, Vec<String>> {
        let mut grouped = HashMap::new();
        for &import in imports {
            let (key, value) = import.split_at(import.find("::").unwrap_or(import.len()));
            grouped.entry(key.to_string()).or_insert_with(Vec::new).push(value.trim_start_matches("::").to_string());
        }
        grouped
    }

    fn format_group((key, values): (&String, &Vec<String>)) -> String {
        if values.len() == 1 {
            format!("use {}{};", key, values[0])
        } else {
            let mut result = String::with_capacity(key.len() + values.iter().map(|s| s.len()).sum::<usize>() + 20);
            result.push_str("use ");
            result.push_str(key);
            result.push_str("::{");
            for (i, value) in values.iter().enumerate() {
                if i > 0 {
                    result.push_str(",\n    ");
                }
                result.push_str(value);
            }
            result.push_str("};");
            result
        }
    }

    let mut organized = Vec::new();
    for (category, imports) in [("Standard library", &std_lib), ("External crate", &external), ("Internal crate", &internal)] {
        if !imports.is_empty() {
            organized.push(format!("// {} imports", category));
            let grouped = group_imports(imports);
            organized.extend(grouped.iter().map(format_group));
            organized.push(String::new());
        }
    }

    organized.extend(non_import_lines.into_iter().map(String::from));
    organized.join("\n")
}
