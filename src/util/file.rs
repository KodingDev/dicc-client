#[cfg(not(target_os = "windows"))]
use std::fs::Permissions;
use std::path::Path;
use tokio::fs;

#[allow(unused_variables)]
pub async fn set_executable(path: &Path) {
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = Permissions::from_mode(0o755);
        fs::set_permissions(path, perms).await.expect("failed to set permissions");
    }
}