use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use syn::{visit::Visit, visit_mut::VisitMut};
use walkdir::WalkDir;

use super::read_file;

pub struct Extractor<'a, 'h> {
    pub lib_path: &'a Path,
    pub current_path: &'a Path,
    pub files: Vec<PathBuf>,
    pub macros: &'h HashMap<String, PathBuf>,
}

impl<'a, 'h> Extractor<'a, 'h> {
    fn add_file(&mut self, ident: &syn::Ident) {
        self.add_path(self.current_path.join(ident.to_string()));
    }

    fn add_path(&mut self, path: PathBuf) {
        let files = convert_sym_path_to_correct_path(path, self.macros);
        self.files.extend(files);
    }

    fn collect_required_files_from_item_use(&mut self, node: &mut syn::ItemUse) {
        assert!(matches!(node.tree, syn::UseTree::Path(_)));
        if let syn::UseTree::Path(ref path) = node.tree {
            if path.ident == "crate" || path.ident == "super" {
                let current_path = if path.ident == "crate" {
                    self.lib_path
                } else {
                    self.current_path
                };
                let mut extracter = Extractor {
                    lib_path: self.lib_path,
                    current_path,
                    files: Vec::new(),
                    macros: self.macros,
                };
                let mut path = path.clone();
                extracter.visit_use_tree_mut(&mut path.tree);
                self.files.extend(extracter.files);
            }
        }
    }

    fn collect_required_files_from_use_path(&mut self, node: &mut syn::UsePath) {
        let mut extracter = Extractor {
            lib_path: self.lib_path,
            current_path: &self.current_path.join(node.ident.to_string()),
            files: Vec::new(),
            macros: self.macros,
        };
        extracter.visit_use_tree_mut(&mut node.tree);
        self.files.extend(extracter.files);
    }

    fn remove_non_use_path(&mut self, node: &mut syn::File) {
        node.items.retain(is_use_path);
    }
}

impl<'a, 'h> VisitMut for Extractor<'a, 'h> {
    fn visit_file_mut(&mut self, node: &mut syn::File) {
        self.remove_non_use_path(node);
        for it in &mut node.items {
            self.visit_item_mut(it);
        }
    }
    fn visit_item_use_mut(&mut self, node: &mut syn::ItemUse) {
        self.collect_required_files_from_item_use(node);
    }

    fn visit_use_path_mut(&mut self, node: &mut syn::UsePath) {
        self.collect_required_files_from_use_path(node);
    }

    fn visit_use_name_mut(&mut self, node: &mut syn::UseName) {
        self.add_file(&node.ident);
    }

    fn visit_use_rename_mut(&mut self, node: &mut syn::UseRename) {
        self.add_file(&node.ident);
    }

    fn visit_use_glob_mut(&mut self, _: &mut syn::UseGlob) {
        self.add_path(self.current_path.to_path_buf());
    }
}

fn is_use_path(item: &syn::Item) -> bool {
    match item {
        syn::Item::Use(ref item) => matches!(item.tree, syn::UseTree::Path(_)),
        _ => false,
    }
}

fn convert_sym_path_to_correct_path<P: AsRef<Path>>(
    sym_path: P,
    macros: &HashMap<String, PathBuf>,
) -> Vec<PathBuf> {
    let path = sym_path.as_ref();

    let mut file = path.to_path_buf();
    file.set_extension("rs");
    if file.is_file() {
        vec![file]
    } else if path.is_dir() {
        WalkDir::new(path)
            .into_iter()
            .filter_entry(|entry| {
                let path = entry.path();
                path.is_file()
                    && path.extension().is_some_and(|e| e == "rs")
                    && path
                        .file_name()
                        .is_some_and(|f| f != "mod.rs" && f != "lib.rs")
            })
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.into_path())
            .collect()
    } else if let Some(file) = path
        .file_stem()
        .and_then(|f| f.to_str())
        .and_then(|f| macros.get(f))
    {
        vec![file.clone()]
    } else if let Some(parent) = path.parent() {
        convert_sym_path_to_correct_path(parent, macros)
    } else {
        Vec::new()
    }
}

struct MacroCollector<'a> {
    pub current_path: &'a Path,
    macros: HashMap<String, PathBuf>,
}

impl<'a> MacroCollector<'a> {
    fn record_macros(&mut self, node: &syn::File) {
        for idx in 0..node.items.len() {
            let item = &node.items[idx];
            if let syn::Item::Macro(item_macro) = item {
                if let Some(name) = &item_macro.ident {
                    self.macros
                        .insert(name.to_string(), self.current_path.to_path_buf());
                }
            }
        }
    }
}

impl<'a, 'ast> Visit<'ast> for MacroCollector<'a> {
    fn visit_file(&mut self, node: &'ast syn::File) {
        self.record_macros(node);
    }
}

pub fn get_all_macros<P: AsRef<Path>>(path: P) -> HashMap<String, PathBuf> {
    let mut macros = HashMap::new();
    for current_path in WalkDir::new(&path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
    {
        let content = read_file(&current_path).unwrap();
        let syntax = syn::parse_file(&content).unwrap();
        let mut collector = MacroCollector {
            current_path: &current_path,
            macros: HashMap::new(),
        };
        collector.visit_file(&syntax);
        macros.extend(collector.macros);
    }
    macros
}
