use std::path::Path;

fn target_is(target: &cargo_metadata::Target, target_kind: &str) -> bool {
    target.kind.iter().any(|kind| kind == target_kind)
}

pub fn select_main_and_libs<P: AsRef<Path>>(
    package_path: P,
) -> (cargo_metadata::Target, Vec<cargo_metadata::Target>) {
    let metadata = get_metadata(package_path);

    let root_package = metadata.root_package().unwrap().clone();
    let main = root_package.targets.first().unwrap().clone();

    let libs = metadata
        .packages
        .into_iter()
        .filter(|package| package.name != root_package.name)
        .flat_map(|package| package.targets)
        .filter(|t| target_is(t, "lib"))
        .collect::<Vec<_>>();

    (main, libs)
}

pub fn get_metadata<P: AsRef<Path>>(package_path: P) -> cargo_metadata::Metadata {
    let manifest_path = package_path.as_ref().join("Cargo.toml");
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.manifest_path(&manifest_path);
    cmd.exec().unwrap()
}
