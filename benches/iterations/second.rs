use std::collections::{HashMap, HashSet};

pub fn organize_rust_imports(code: &str) -> String {
    let mut imports = Vec::new();
    let mut non_import_lines = Vec::new();
    let mut in_import_section = false;

    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("use ") {
            imports.push(trimmed);
            in_import_section = true;
        } else if in_import_section && !trimmed.is_empty() {
            in_import_section = false;
        }
        if !in_import_section {
            non_import_lines.push(line.to_string());
        }
    }

    let mut std_lib = Vec::new();
    let mut external = Vec::new();
    let mut internal = Vec::new();

    for import in imports {
        if import.starts_with("use std::") {
            std_lib.push(import);
        } else if import.starts_with("use crate::") {
            internal.push(import);
        } else {
            external.push(import);
        }
    }

    fn group_imports(imports: &[&str]) -> HashMap<String, HashSet<String>> {
        let mut grouped = HashMap::new();
        for &import in imports {
            let (key, value) = import.split_at(import.find("::").unwrap_or(import.len()));
            grouped.entry(key.to_string()).or_insert_with(HashSet::new).insert(value.trim_start_matches("::").to_string());
        }
        grouped
    }

    fn format_group((key, values): (&String, &HashSet<String>)) -> String {
        if values.len() == 1 {
            format!("use {}{};", key, values.iter().next().unwrap())
        } else {
            let items = values.iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(",\n    ");
            format!("use {}::{{
    {}
}};", key, items)
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

    organized.extend(non_import_lines);
    organized.join("\n")
}