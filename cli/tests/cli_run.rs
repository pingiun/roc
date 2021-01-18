// #[macro_use]
extern crate pretty_assertions;

extern crate bumpalo;
extern crate inlinable_string;
extern crate roc_collections;
extern crate roc_load;
extern crate roc_module;

mod helpers;

#[cfg(test)]
mod cli_run {
    use crate::helpers::{
        example_file, extract_valgrind_errors, fixture_file, run_cmd, run_roc, run_with_valgrind,
    };
    use serial_test::serial;
    use std::path::Path;

    fn check_output(
        file: &Path,
        executable_filename: &str,
        flags: &[&str],
        expected_ending: &str,
        use_valgrind: bool,
    ) {
        let compile_out = run_roc(&[&["build", file.to_str().unwrap()], flags].concat());
        if !compile_out.stderr.is_empty() {
            panic!(compile_out.stderr);
        }
        assert!(compile_out.status.success());

        let out = if use_valgrind {
            let (valgrind_out, raw_xml) =
                run_with_valgrind(&[file.with_file_name(executable_filename).to_str().unwrap()]);

            if valgrind_out.status.success() {
                let memory_errors = extract_valgrind_errors(&raw_xml).unwrap_or_else(|err| {
                    panic!("failed to parse the `valgrind` xml output. Error was:\n\n{:?}\n\nvalgrind xml was: \"{}\"\n\nvalgrind stdout was: \"{}\"\n\nvalgrind stderr was: \"{}\"", err, raw_xml, valgrind_out.stdout, valgrind_out.stderr);
                });

                if !memory_errors.is_empty() {
                    panic!("{:?}", memory_errors);
                }
            } else {
                let exit_code = match valgrind_out.status.code() {
                    Some(code) => format!("exit code {}", code),
                    None => "no exit code".to_string(),
                };

                panic!("`valgrind` exited with {}. valgrind stdout was: \"{}\"\n\nvalgrind stderr was: \"{}\"", exit_code, valgrind_out.stdout, valgrind_out.stderr);
            }

            valgrind_out
        } else {
            run_cmd(
                file.with_file_name(executable_filename).to_str().unwrap(),
                &[],
            )
        };
        if !&out.stdout.ends_with(expected_ending) {
            panic!(
                "expected output to end with {:?} but instead got {:#?}",
                expected_ending, out
            );
        }
        assert!(out.status.success());
    }

    #[test]
    #[serial(hello_world)]
    fn run_hello_world() {
        check_output(
            &example_file("hello-world", "Hello.roc"),
            "hello-world",
            &[],
            "Hello, World!!!!!!!!!!!!!\n",
            true,
        );
    }

    #[test]
    #[serial(hello_world)]
    fn run_hello_world_optimized() {
        check_output(
            &example_file("hello-world", "Hello.roc"),
            "hello-world",
            &[],
            "Hello, World!!!!!!!!!!!!!\n",
            true,
        );
    }

    #[test]
    #[serial(quicksort)]
    fn run_quicksort_not_optimized() {
        check_output(
            &example_file("quicksort", "Quicksort.roc"),
            "quicksort",
            &[],
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2]\n",
            false,
        );
    }

    #[test]
    #[serial(quicksort)]
    fn run_quicksort_optimized() {
        check_output(
            &example_file("quicksort", "Quicksort.roc"),
            "quicksort",
            &["--optimize"],
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2]\n",
            false,
        );
    }

    #[test]
    #[serial(quicksort)]
    // TODO: Stop ignoring this test once we are correctly freeing the RocList even when in dev build.
    #[ignore]
    fn run_quicksort_valgrind() {
        check_output(
            &example_file("quicksort", "Quicksort.roc"),
            "quicksort",
            &[],
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2]\n",
            true,
        );
    }

    #[test]
    #[serial(quicksort)]
    // TODO: Stop ignoring this test once valgrind supports AVX512.
    #[ignore]
    fn run_quicksort_optimized_valgrind() {
        check_output(
            &example_file("quicksort", "Quicksort.roc"),
            "quicksort",
            &["--optimize"],
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2]\n",
            true,
        );
    }

    #[test]
    #[serial(multi_module)]
    // TODO: Stop ignoring this test once we are correctly freeing the RocList even when in dev build.
    #[ignore]
    fn run_multi_module_valgrind() {
        check_output(
            &example_file("multi-module", "Quicksort.roc"),
            "quicksort",
            &[],
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2]\n",
            true,
        );
    }

    #[test]
    #[serial(multi_module)]
    // TODO: Stop ignoring this test once valgrind supports AVX512.
    #[ignore]
    fn run_multi_module_optimized_valgrind() {
        check_output(
            &example_file("multi-module", "Quicksort.roc"),
            "quicksort",
            &["--optimize"],
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2]\n",
            true,
        );
    }

    #[test]
    #[serial(nqueens)]
    fn run_nqueens_not_optimized() {
        check_output(
            &example_file("benchmarks", "NQueens.roc"),
            "nqueens",
            &[],
            "724\n",
            false,
        );
    }

    #[test]
    #[serial(cfold)]
    fn run_cfold_not_optimized() {
        check_output(
            &example_file("benchmarks", "CFold.roc"),
            "cfold",
            &[],
            "11 & 11\n",
            false,
        );
    }

    //    #[test]
    //    #[serial(effect)]
    //    fn run_effect_unoptimized() {
    //        check_output(
    //            &example_file("effect", "Main.roc"),
    //            &[],
    //            "I am Dep2.str2\n",
    //            true,
    //        );
    //    }

    #[test]
    #[serial(multi_dep_str)]
    fn run_multi_dep_str_unoptimized() {
        check_output(
            &fixture_file("multi-dep-str", "Main.roc"),
            "multi-dep-str",
            &[],
            "I am Dep2.str2\n",
            true,
        );
    }

    #[test]
    #[serial(multi_dep_str)]
    fn run_multi_dep_str_optimized() {
        check_output(
            &fixture_file("multi-dep-str", "Main.roc"),
            "multi-dep-str",
            &["--optimize"],
            "I am Dep2.str2\n",
            true,
        );
    }

    #[test]
    #[serial(multi_dep_thunk)]
    fn run_multi_dep_thunk_unoptimized() {
        check_output(
            &fixture_file("multi-dep-thunk", "Main.roc"),
            "multi-dep-thunk",
            &[],
            "I am Dep2.value2\n",
            true,
        );
    }

    #[test]
    #[serial(multi_dep_thunk)]
    fn run_multi_dep_thunk_optimized() {
        check_output(
            &fixture_file("multi-dep-thunk", "Main.roc"),
            "multi-dep-thunk",
            &["--optimize"],
            "I am Dep2.value2\n",
            true,
        );
    }
}
