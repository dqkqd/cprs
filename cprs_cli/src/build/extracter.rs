use std::path::{Path, PathBuf};

use syn::visit_mut::VisitMut;
use walkdir::WalkDir;

pub struct Extracter<'a> {
    pub lib_path: &'a Path,
    pub current_path: &'a Path,
    pub files: Vec<PathBuf>,
}

impl<'a> Extracter<'a> {
    fn add_file(&mut self, ident: &syn::Ident) {
        self.add_path(self.current_path.join(ident.to_string()));
    }

    fn add_path(&mut self, path: PathBuf) {
        let files = convert_sym_path_to_correct_path(path);
        self.files.extend(files);
    }

    fn collect_required_files_from_item_use(&mut self, node: &mut syn::ItemUse) {
        assert!(matches!(node.tree, syn::UseTree::Path(_)));
        if let syn::UseTree::Path(ref path) = node.tree {
            if path.ident == "crate" || path.ident == "super" {
                let mut extracter = Extracter {
                    lib_path: self.lib_path,
                    current_path: self.current_path,
                    files: Vec::new(),
                };
                let mut path = path.clone();
                extracter.visit_use_tree_mut(&mut path.tree);
                self.files.extend(extracter.files);
            }
        }
    }

    fn collect_required_files_from_use_path(&mut self, node: &mut syn::UsePath) {
        let mut extracter = Extracter {
            lib_path: self.lib_path,
            current_path: &self.current_path.join(node.ident.to_string()),
            files: Vec::new(),
        };
        extracter.visit_use_tree_mut(&mut node.tree);
        self.files.extend(extracter.files);
    }

    fn remove_non_use_path(&mut self, node: &mut syn::File) {
        node.items.retain(is_use_path);
    }
}

impl<'a> VisitMut for Extracter<'a> {
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

fn convert_sym_path_to_correct_path<P: AsRef<Path>>(sym_path: P) -> Vec<PathBuf> {
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
    } else if let Some(parent) = path.parent() {
        convert_sym_path_to_correct_path(parent)
    } else {
        Vec::new()
    }
}
