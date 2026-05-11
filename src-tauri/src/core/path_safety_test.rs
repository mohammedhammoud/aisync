use crate::core::path_safety::{expand_home, validate_id};
use crate::platform::platform;
use crate::test_support::{temp_root, test_lock};

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
