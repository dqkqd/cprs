use anyhow::{Context, Result};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::DerefMut,
    path::{Path, PathBuf},
};
use syn::{__private::ToTokens, punctuated::Punctuated, visit_mut::VisitMut};
use walkdir::WalkDir;

use crate::{
    build::{loader::select_main_and_libs, prettify, read_file},
    task::Task,
};

use super::extracter::{get_all_macros, Extractor};

pub fn bundle_task(task: &Task) -> Result<String> {
    let (main, libs) = select_main_and_libs(&task.task_folder);
    let content = read_file(&main.src_path)?;
    let mut syntax_tree = syn::parse_file(&content)?;

    let mut macros = HashMap::new();
    for lib in libs {
        let lib_path = lib
            .src_path
            .parent()
            .with_context(|| format!("Cannot file parent for {}", lib.src_path))?
            .as_std_path();

        let lib_macros = get_all_macros(lib_path);

        let config = BundlerConfig {
            remove_tests: false,
            move_tests_to_the_end: true,
            rename_crate_to_lib_name: false,
        };
        let mut bundler = Bundler::with_config(config)
            .with_lib(&lib.name)
            .with_path(lib_path)
            .with_macro(lib_macros);
        bundler.visit_file_mut(&mut syn::parse_file(&content)?);

        macros.extend(bundler.macros.into_iter());
        let required_files = bundler.required_files;
        if let Ok(lib_mod) = create_mod(&lib.name, lib_path, &lib.name, &required_files) {
            syntax_tree.items.push(lib_mod);
        }
    }

    let config = BundlerConfig {
        remove_tests: false,
        move_tests_to_the_end: true,
        rename_crate_to_lib_name: false,
    };
    Bundler::with_config(config)
        .with_macro(macros)
        .visit_file_mut(&mut syntax_tree);

    let code = syntax_tree.into_token_stream().to_string();
    let code = prettify(&code)?;
    Ok(code)
}

#[derive(Default)]
pub struct BundlerConfig {
    remove_tests: bool,
    move_tests_to_the_end: bool,
    rename_crate_to_lib_name: bool,
}

#[derive(Default)]
pub struct Bundler<'s> {
    current_lib: Option<&'s str>,
    current_path: Option<&'s Path>,
    config: BundlerConfig,
    required_files: Vec<PathBuf>,
    macros: HashMap<String, PathBuf>,
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

    fn with_macro(mut self, macros: HashMap<String, PathBuf>) -> Bundler<'s> {
        self.macros = macros;
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
        let mut extracter = Extractor {
            lib_path: path,
            current_path: path,
            files: Vec::new(),
            macros: &mut self.macros,
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
            let mut extracter = Extractor {
                lib_path: path,
                current_path: p.parent().unwrap(),
                files: Vec::new(),
                macros: &mut self.macros,
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

    fn contains_macro_name(&mut self, node: &syn::UseTree) -> bool {
        match node {
            syn::UseTree::Name(name) => self.macros.contains_key(&name.ident.to_string()),
            _ => false,
        }
    }

    fn handle_remove_use_macro_in_file(&mut self, node: &mut syn::File) {
        node.items.retain(|item| match item {
            syn::Item::Use(item) => !self.contains_macro_name(&item.tree),
            _ => true,
        })
    }

    fn handle_remove_use_macro_in_group(&mut self, node: &mut syn::UseGroup) {
        node.items = node
            .items
            .clone()
            .into_iter()
            .filter(|item| !self.contains_macro_name(item))
            .collect();
    }
    fn handle_rename_crate_macro(&mut self, node: &mut syn::UsePath) {
        // rename crate::*::macro to crate::macro
        if node.ident == "crate" {
            if let syn::UseTree::Path(use_path) = node.tree.as_ref() {
                if let syn::UseTree::Name(use_name) = use_path.tree.as_ref() {
                    if self.macros.contains_key(&use_name.ident.to_string()) {
                        // before:
                        // node = crate::*::macro:
                        // after swap
                        // node = crate::macro
                        std::mem::swap(use_path.clone().tree.deref_mut(), node.tree.deref_mut());
                    }
                }
            }
        }
    }
    fn handle_rename_crate_to_lib(&mut self, node: &mut syn::UsePath) {
        if self.config.rename_crate_to_lib_name
            && node.ident == "crate"
            && self.current_lib.is_some()
        {
            let code = format!("{}::*", self.current_lib.unwrap());
            match syn::parse_str::<syn::UseTree>(&code) {
                Ok(syn::UseTree::Path(mut new_use_path)) => {
                    // before:
                    // code = lib_name::*
                    // node = crate::something
                    // node.tree = something
                    //
                    // after swap
                    // code = lib_name::something
                    // node = crate::*
                    // node.tree = *
                    //
                    // after replace
                    // code = Empty
                    // node = crate::lib_name::something
                    // node.tree = lib_name::something
                    std::mem::swap(new_use_path.tree.deref_mut(), node.tree.deref_mut());
                    let _ =
                        std::mem::replace(node.tree.deref_mut(), syn::UseTree::Path(new_use_path));
                }
                _ => unreachable!(),
            }
        }
    }
}

impl<'s> VisitMut for Bundler<'s> {
    fn visit_file_mut(&mut self, node: &mut syn::File) {
        for it in &mut node.attrs {
            self.visit_attribute_mut(it);
        }
        self.handle_tests(node);
        self.handle_remove_use_macro_in_file(node);
        for it in &mut node.items {
            self.visit_item_mut(it);
        }
    }

    fn visit_use_path_mut(&mut self, node: &mut syn::UsePath) {
        self.handle_rename_crate_to_lib(node);
        self.handle_rename_crate_macro(node);
        self.visit_ident_mut(&mut node.ident);
        self.collect_required_files(node);
        self.visit_use_tree_mut(&mut node.tree);
    }

    fn visit_use_group_mut(&mut self, node: &mut syn::UseGroup) {
        self.handle_remove_use_macro_in_group(node);
        for mut el in Punctuated::pairs_mut(&mut node.items) {
            let it = el.value_mut();
            self.visit_use_tree_mut(it);
        }
    }
}

fn create_mod<P: AsRef<Path>>(
    lib_name: &str,
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
            rename_crate_to_lib_name: true,
        };
        Bundler::with_config(config)
            .with_lib(lib_name)
            .with_path(base_path.as_ref())
            .visit_file_mut(&mut syntax);

        items.extend(syntax.items);
    }

    for entry in WalkDir::new(&base_path)
        .sort_by_file_name()
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
            .filter_map(|_| create_mod(lib_name, entry.path(), local_name, required_files).ok())
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
