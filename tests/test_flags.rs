// Integration tests for flags module
// Each test module is in a separate file under tests/flags/

#[path = "flags/test_size.rs"]
mod test_size;

#[path = "flags/test_indicators.rs"]
mod test_indicators;

#[path = "flags/test_icons.rs"]
mod test_icons;

#[path = "flags/test_display.rs"]
mod test_display;

#[path = "flags/test_layout.rs"]
mod test_layout;

#[path = "flags/test_permission.rs"]
mod test_permission;

#[path = "flags/test_symlinks.rs"]
mod test_symlinks;

#[path = "flags/test_literal.rs"]
mod test_literal;

#[path = "flags/test_dereference.rs"]
mod test_dereference;

#[path = "flags/test_hyperlink.rs"]
mod test_hyperlink;

#[path = "flags/test_sorting.rs"]
mod test_sorting;

#[path = "flags/test_header.rs"]
mod test_header;

#[path = "flags/test_truncate_owner.rs"]
mod test_truncate_owner;

#[path = "flags/test_total_size.rs"]
mod test_total_size;

// NOTE: The following tests use pub(crate) methods and are in separate integration test files:
// - test_ignore_globs.rs
// - test_recursion.rs
// - test_symlink_arrow.rs
