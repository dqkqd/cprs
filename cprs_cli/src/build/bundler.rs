use anyhow::{Context, Result};
use std::{
    collections::{HashSet, VecDeque},
    path::{Path, PathBuf},
};
use syn::{__private::ToTokens, visit_mut::VisitMut};
use walkdir::WalkDir;

use crate::{
    build::{loader::select_main_and_libs, prettify, read_file},
    task::Task,
};

use super::extracter::Extracter;

pub fn bundle_task(task: &Task) -> Result<String> {
    let (main, libs) = select_main_and_libs(&task.task_folder);
    let content = read_file(&main.src_path)?;
    let mut syntax_tree = syn::parse_file(&content)?;
    for lib in libs {
        let lib_path = lib
            .src_path
            .parent()
            .with_context(|| format!("Cannot file parent for {}", lib.src_path))?
            .as_std_path();

        let config = BundlerConfig {
            remove_tests: false,
            move_tests_to_the_end: true,
        };
        let mut bundler = Bundler::with_config(config)
            .with_lib(&lib.name)
            .with_path(lib_path);
        bundler.visit_file_mut(&mut syn::parse_file(&content)?);

        let required_files = bundler.required_files;
        if let Ok(lib_mod) = create_mod(lib_path, &lib.name, &required_files) {
            syntax_tree.items.push(lib_mod);
        }
    }

    let config = BundlerConfig {
        remove_tests: false,
        move_tests_to_the_end: true,
    };
    Bundler::with_config(config).visit_file_mut(&mut syntax_tree);

    let code = syntax_tree.into_token_stream().to_string();
    let code = prettify(&code)?;
    Ok(code)
}

#[derive(Default)]
pub struct BundlerConfig {
    remove_tests: bool,
    move_tests_to_the_end: bool,
}

#[derive(Default)]
pub struct Bundler<'s> {
    current_lib: Option<&'s str>,
    current_path: Option<&'s Path>,
    config: BundlerConfig,
    required_files: Vec<PathBuf>,
}

impl<'s> Bundler<'s> {
    fn with_config(config: BundlerConfig) -> Bundler<'s> {
        Bundler {
            config,
            ..Default::default()
        }
    }

    fn with_lib(mut self, lib_name: &'s str) -> Bundler<'s> {
        self.current_lib = Some(lib_name);
        self
    }

    fn with_path(mut self, path: &'s Path) -> Bundler<'s> {
        self.current_path = Some(path);
        self
    }

    fn extract_test_nodes(&mut self, node: &mut syn::File) -> Vec<syn::Item> {
        let test_nodes = node
            .items
            .iter()
            .filter(|item| is_attr_test(item))
            .cloned()
            .collect::<Vec<_>>();
        node.items.retain(|item| !is_attr_test(item));
        test_nodes
    }

    fn collect_required_files(&mut self, node: &mut syn::UsePath) {
        if self.current_lib.is_none() {
            return;
        }
        if self.current_path.is_none() {
            return;
        }

        let lib = self.current_lib.unwrap();

        let ident = node.ident.to_string();
        if ident != lib {
            return;
        }

        let mut required_files: HashSet<PathBuf> = HashSet::new();
        let path = self.current_path.unwrap();
        let mut extracter = Extracter {
            lib_path: path,
            current_path: path,
            files: Vec::new(),
        };
        extracter.visit_use_tree_mut(&mut node.tree.clone());

        let mut processing_paths = VecDeque::new();
        processing_paths.extend(extracter.files);

        while let Some(p) = processing_paths.pop_front() {
            if required_files.contains(&p) {
                continue;
            }
            assert!(p.is_file());
            let content = read_file(&p).unwrap();
            let mut syntax = syn::parse_file(&content).unwrap();
            let mut extracter = Extracter {
                lib_path: path,
                current_path: p.parent().unwrap(),
                files: Vec::new(),
            };
            extracter.visit_file_mut(&mut syntax);
            processing_paths.extend(extracter.files);
            required_files.insert(p);
        }

        self.required_files = required_files.into_iter().collect();
        self.required_files.sort();
    }

    fn handle_tests(&mut self, node: &mut syn::File) {
        if self.config.remove_tests {
            self.extract_test_nodes(node);
        } else if self.config.move_tests_to_the_end {
            let test_nodes = self.extract_test_nodes(node);
            node.items.extend(test_nodes);
        }
    }
}

impl<'s> VisitMut for Bundler<'s> {
    fn visit_file_mut(&mut self, node: &mut syn::File) {
        for it in &mut node.attrs {
            self.visit_attribute_mut(it);
        }
        self.handle_tests(node);
        for it in &mut node.items {
            self.visit_item_mut(it);
        }
    }

    fn visit_use_path_mut(&mut self, node: &mut syn::UsePath) {
        self.visit_ident_mut(&mut node.ident);
        self.collect_required_files(node);
        self.visit_use_tree_mut(&mut node.tree);
    }
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
        let mut syntax = syn::parse_file(&content)?;

        let config = BundlerConfig {
            remove_tests: true,
            move_tests_to_the_end: false,
        };
        Bundler::with_config(config).visit_file_mut(&mut syntax);

        items.extend(syntax.items);
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
