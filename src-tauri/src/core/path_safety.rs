mod ids;
mod paths;
mod types;

pub use ids::validate_id;
pub use paths::{app_root, assert_child, expand_home};
pub use types::PathErrorCode;

#[cfg(test)]
mod tests {
    use crate::core::path_safety::{assert_child, expand_home, validate_id};
    use crate::platform::platform;
    use crate::test_support::{set_home, temp_root, test_lock};

    #[test]
    fn expand_home_uses_platform_home() {
        let _guard = test_lock();
        let root = temp_root("expand-home");
        let previous_home = std::env::var_os("HOME");
        std::env::set_var("HOME", &root);

        assert_eq!(platform().home_dir(), root);
        assert_eq!(expand_home("~/nested"), root.join("nested"));

        if let Some(previous_home) = previous_home {
            std::env::set_var("HOME", previous_home);
        } else {
            std::env::remove_var("HOME");
        }
    }

    #[test]
    fn invalid_ids_are_rejected() {
        assert!(validate_id("../bad").is_err());
        assert!(validate_id("bad/slash").is_err());
        assert!(validate_id("good-id_1").is_ok());
    }

    #[test]
    fn assert_child_rejects_symlink_parent_escape_for_new_path() {
        let _guard = test_lock();
        let root = temp_root("path-symlink-root");
        let outside = temp_root("path-symlink-outside");
        set_home(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        platform()
            .symlink_path(&outside, &root.join("linked-outside"))
            .unwrap();

        let error = assert_child(&root, &root.join("linked-outside").join("new-file")).unwrap_err();

        assert_eq!(
            error.code,
            crate::core::errors::AppErrorCode::Path(super::PathErrorCode::PathEscapesRoot)
        );
    }
}
