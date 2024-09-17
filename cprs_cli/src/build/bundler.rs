use anyhow::{Context, Result};
use std::{
    collections::{HashSet, VecDeque},
    path::{Path, PathBuf},
};
use syn::__private::ToTokens;
use walkdir::WalkDir;

use crate::{
    build::{loader::select_main_and_libs, prettify, read_file},
    task::Task,
};

pub fn bundle_task(task: &Task) -> Result<String> {
    let (main, libs) = select_main_and_libs(&task.task_folder);
    let content = read_file(&main.src_path)?;
    let mut syntax_tree = syn::parse_file(&content)?;
    let main_path = main.src_path.as_std_path();
    for lib in libs {
        let lib_path = lib
            .src_path
            .parent()
            .with_context(|| format!("Cannot file parent for {}", lib.src_path))?
            .as_std_path();
        let required_files = get_required_files(main_path, lib_path, &lib.name);
        if let Ok(lib_mod) = create_mod(lib_path, &lib.name, &required_files) {
            syntax_tree.items.push(lib_mod);
        }
    }
    let code = syntax_tree.into_token_stream().to_string();
    let code = prettify(&code)?;
    Ok(code)
}

fn create_mod<P: AsRef<Path>>(
    base_path: P,
    mod_name: &str,
    required_files: &[PathBuf],
) -> Result<syn::Item> {
    let mut item_mod: syn::ItemMod = syn::parse_str(&format!("pub mod {} {{}}", mod_name))?;
    let mut items = Vec::new();

    let mut file = base_path.as_ref().to_path_buf();
    file.set_extension("rs");
    if file.is_file() {
        let content = read_file(&file)?;
        let syntax = syn::parse_file(&content)?;
        items.extend(syntax.items.into_iter().filter(|item| !is_attr_test(item)))
    }

    for entry in WalkDir::new(&base_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let local_name = entry
            .path()
            .file_stem()
            .and_then(|f| f.to_str())
            .with_context(|| "Cannot get file stem")?;
        if local_name == "mod" || local_name == "lib" {
            continue;
        }

        let local_mod = required_files
            .iter()
            .filter(|file| file.starts_with(entry.path()))
            .filter_map(|_| create_mod(entry.path(), local_name, required_files).ok())
            .next();
        if let Some(local_mod) = local_mod {
            items.push(local_mod);
        }
    }

    if !items.is_empty() {
        item_mod.content = item_mod.content.map(|(brace, _)| (brace, items));
    }
    Ok(syn::Item::Mod(item_mod))
}

fn get_required_files<P1: AsRef<Path>, P2: AsRef<Path>>(
    entry_file: P1,
    lib_path: P2,
    prefix: &str,
) -> Vec<PathBuf> {
    let get_required_files = |file: &Path| -> Vec<PathBuf> {
        get_use_trees(file, prefix)
            .iter()
            .flat_map(|item| convert_use_tree_to_files(&lib_path, item))
            .collect()
    };

    let rust_files = get_required_files(entry_file.as_ref());
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from_iter(rust_files);
    while !queue.is_empty() {
        let file = queue.pop_front().unwrap();
        if seen.contains(&file) {
            continue;
        }
        let required_files = get_required_files(file.as_ref());
        queue.extend(required_files);
        seen.insert(file);
    }
    seen.into_iter().collect()
}

fn get_use_trees<P: AsRef<Path>>(file: P, prefix: &str) -> Vec<syn::UseTree> {
    let content = read_file(file).unwrap();
    let syntax = syn::parse_file(&content).unwrap();
    syntax
        .items
        .into_iter()
        .filter_map(|item| {
            if let syn::Item::Use(item_use) = item {
                if let syn::UseTree::Path(path) = item_use.tree {
                    if path.ident == prefix || path.ident == "crate" {
                        return Some(*path.tree);
                    }
                }
            }
            None
        })
        .collect()
}

fn convert_use_tree_to_files<P: AsRef<Path>>(base_path: P, item: &syn::UseTree) -> Vec<PathBuf> {
    let mut files = Vec::new();
    match item {
        syn::UseTree::Path(path) => {
            files.extend(convert_use_tree_to_files(
                base_path.as_ref().join(path.ident.to_string()),
                &path.tree,
            ));
        }
        syn::UseTree::Name(name) => {
            files.extend(grep_rust_files(
                base_path.as_ref().join(name.ident.to_string()),
            ));
        }
        syn::UseTree::Rename(name) => {
            files.extend(grep_rust_files(
                base_path.as_ref().join(name.ident.to_string()),
            ));
        }
        syn::UseTree::Glob(_) => {}
        syn::UseTree::Group(group) => {
            files.extend(
                group
                    .items
                    .iter()
                    .flat_map(|item| convert_use_tree_to_files(base_path.as_ref(), item)),
            );
        }
    };

    if files.is_empty() {
        files.extend(grep_rust_files(base_path))
    }
    files
}

fn grep_rust_files<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut rust_files = Vec::new();
    let mut file = path.as_ref().to_path_buf();
    file.set_extension("rs");
    if file.is_file() {
        rust_files.push(file);
    } else if path.as_ref().is_dir() && path.as_ref().join("mod.rs").is_file() {
        rust_files = WalkDir::new(path)
            .into_iter()
            .filter_entry(|entry| {
                let path = entry.path();
                path.is_file()
                    && path.extension().is_some_and(|e| e == "rs")
                    && path.file_name().is_some_and(|f| f != "mod.rs")
            })
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.into_path())
            .collect()
    }
    rust_files
}

fn is_attr_test(item: &syn::Item) -> bool {
    match item {
        syn::Item::Mod(item_mod) => item_mod.attrs.iter().any(|attr| match attr.meta {
            syn::Meta::List(ref meta_list) => {
                let is_test_token = meta_list.tokens.to_string() == "test";
                let is_cfg_test = meta_list
                    .path
                    .segments
                    .iter()
                    .any(|segment| segment.ident == "cfg");
                is_test_token && is_cfg_test
            }
            _ => false,
        }),
        _ => false,
    }
}
