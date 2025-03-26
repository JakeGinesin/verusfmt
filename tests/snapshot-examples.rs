//! Automatic tests for various files in ../examples/
//!
//! This is essentially intended to be a snapshot test, like ./verus-consistency.rs, but only as a
//! quick indicator for whether files in `../examples/` (such as `../examples/syntax.rs`) have been
//! modified by any change.

fn verusfmt_run_with_extra_stack(s: &str, opts: verusfmt::RunOptions) -> miette::Result<String> {
    #[allow(non_upper_case_globals)]
    const MiB: usize = 1024 * 1024;
    const STACK_SIZE: usize = 8 * MiB;
    stacker::grow(STACK_SIZE, || verusfmt::run(s, opts))
}

fn check_snapshot(original: &str) {
    check_snapshot_with_config(original, Default::default())
}

fn check_snapshot_with_config(original: &str, config: verusfmt::RunOptions) {
    let formatted = verusfmt_run_with_extra_stack(original, config).unwrap();
    if original != formatted {
        let diff = similar::udiff::unified_diff(
            similar::Algorithm::Patience,
            original,
            &formatted,
            3,
            Some(("original", "formatted")),
        );
        println!("{diff}");
        panic!("Formatted output does not match");
    }
}

#[test]
fn atomic_rs_unchanged() {
    check_snapshot(include_str!("../examples/atomic.rs"));
}

#[glob_macro::glob("./examples/ironfleet-snapshot/**/*.rs")]
#[test]
fn ironfleet_snapshot_unchanged(path: &std::path::Path) {
    if std::env::var("RUN_SLOW_TESTS").is_err()
        && matches!(
            path.file_name().unwrap().to_str().unwrap(),
            "delegation_map_v.rs" | "host_impl_v.rs"
        )
    {
        // Skip slow tests. The CI enables the flag however, so it _will_ run on CI. See
        // https://matklad.github.io/2021/05/31/how-to-test.html#Make-Tests-Fast for this idea.
        return;
    }
    check_snapshot(&std::fs::read_to_string(path).unwrap());
}

#[test]
#[ignore] // Due to "fatal runtime error: stack overflow" during `cargo test`, and comment failure during regular execution
fn mimalloc_rs_unchanged() {
    check_snapshot(include_str!("../examples/mimalloc.rs"));
}

#[glob_macro::glob("./examples/nr-snapshot/**/*.rs")]
#[test]
fn nr_unchanged(path: &std::path::Path) {
    check_snapshot(&std::fs::read_to_string(path).unwrap());
}

#[test]
fn owl_output_rs_unchanged() {
    check_snapshot(include_str!("../examples/owl-output.rs"));
}

#[glob_macro::glob("./examples/pagetable-snapshot/**/*.rs")]
#[test]
fn pagetable_unchanged(path: &std::path::Path) {
    if std::env::var("RUN_SLOW_TESTS").is_err()
        && matches!(
            path.file_name().unwrap().to_str().unwrap(),
            "l2_impl.rs" | "l2_refinement.rs" | "os_refinement.rs" | "l1.rs"
        )
    {
        // Skip slow tests. The CI enables the flag however, so it _will_ run on CI. See
        // https://matklad.github.io/2021/05/31/how-to-test.html#Make-Tests-Fast for this idea.
        return;
    }
    check_snapshot(&std::fs::read_to_string(path).unwrap());
}

#[glob_macro::glob("./examples/verus-snapshot/**/*.rs")]
#[test]
fn verus_snapshot_unchanged(path: &std::path::Path) {
    check_snapshot_with_config(
        &std::fs::read_to_string(path).unwrap(),
        verusfmt::RunOptions {
            file_name: None,
            run_rustfmt: true,
            rustfmt_config: verusfmt::RustFmtConfig {
                rustfmt_toml: Some(
                    std::fs::read_to_string("./examples/verus-snapshot/source/rustfmt.toml")
                        .unwrap(),
                ),
            },
        },
    );
}
