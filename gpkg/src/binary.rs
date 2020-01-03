use crate::storage::Metadata;
use std::path::Path;

pub struct Binary<P1: AsRef<Path>, P2: AsRef<Path>, NodePath: AsRef<Path>> {
    pub metadata: Metadata,
    pub symlink_path: P1,
    pub target_path: P2,
    pub node_binary_path: NodePath,
}

impl<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>> Binary<P1, P2, P3> {
    pub fn new(metadata: Metadata, symlink_path: P1, target_path: P2, node_path: P3) -> Self {
        Self {
            metadata,
            symlink_path,
            target_path,
            node_binary_path: node_path,
        }
    }

    pub fn script_src(&self) -> String {
        let binary_path = self
            .node_binary_path
            .as_ref()
            .parent()
            .expect("Got node with no parent");
        let source = format!(
            r#"
                #!/bin/sh
                # metadata: {metadata_json}
                export PATH={node_binary_path:?}:$PATH
                {binary_path:?} "$@"
            "#,
            metadata_json = base64::encode(&serde_json::to_string(&self.metadata).unwrap()),
            binary_path = self.target_path.as_ref(),
            node_binary_path = binary_path,
        );
        unindent::unindent(&source)
    }

    pub fn create_script(self) -> std::io::Result<P1> {
        let src = self.script_src();
        std::fs::write(&self.symlink_path, &src)?;
        set_permissions(&self.symlink_path)?;

        Ok(self.symlink_path)
    }
}

#[cfg(unix)]
fn set_permissions(script_path: impl AsRef<Path>) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = script_path.as_ref().metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o744); // set executable
    std::fs::set_permissions(&script_path, permissions)?;
    Ok(())
}

#[cfg(windows)]
fn set_permissions(_script_path: impl AsRef<Path>) -> std::io::Result<()> {
    Ok(())
}
