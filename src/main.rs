use std::collections::HashMap;
use std::io::{self, Read, Write};

fn organize_rust_imports(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut imports = Vec::new();
    let mut organized = Vec::new();

    for line in &lines {
        if line.trim().starts_with("use ") {
            imports.push(line.trim());
        } else if !imports.is_empty() {
            break;
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

    fn group_imports(imports: &[&str]) -> HashMap<String, Vec<String>> {
        let mut grouped = HashMap::new();
        for &import in imports {
            let parts: Vec<&str> = import.split("::").collect();
            let key = parts[..2].join("::");
            let value = parts[..2].join("::");
            grouped.entry(key).or_insert_with(Vec::new).push(value)
        }
        grouped
    }

    fn format_group(group: (&String, &Vec<String>)) -> String {
        if group.len() == 1 {
            format!("use {};", group.0)
        } else {
            let items = group.1.join(",\n   ");
            format!("use {} {{\n    {}\n}}", group.0, items)
        }
    }

    if !std_lib.is_empty() {
        organized.push("// standard library imports".to_string());
        let grouped = group_imports(&std_lib);
        for group in grouped.iter().map(format_group) {
            organized.push(group);
        }
        organized.push(String::new());
    }

    if !external.is_empty() {
        organized.push("// External crate imports".to_string());
        let grouped = group_imports(&external);
        for group in grouped.iter().map(format_group) {
            organized.push(group);
        }
        organized.push(String::new());
    }

    if !internal.is_empty() {
        organized.push("// Internal crate imports".to_string());
        let grouped = group_imports(&internal);
        for group in grouped.iter().map(format_group) {
            organized.push(group);
        }
    }

    let mut result = Vec::new();
    let mut import_section_passed = false;

    for line in lines {
        if !import_section_passed {
            if line.trim().starts_with("use ") {
                if result.is_empty() {
                    result.extend(organized);
                }
                import_section_passed = true;
            } else {
                result.push(line.to_string());
            }
        } else {
            result.push(line.to_string())
        }
    }
    result.join("\n")
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let organized = organize_rust_imports(&input);
    io::stdout().write_all(organized.as_bytes())?;

    Ok(())
}