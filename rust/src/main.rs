use std::string::String;

use pyo3::prelude::PyModule;
use pyo3::{pyclass, pyfunction, pymethods, pymodule, wrap_pyfunction, Bound, PyResult};
use taplo::formatter::{format_syntax, Options};
use taplo::parser::parse;

use crate::global::reorder_tables;
use crate::helpers::table::Tables;

mod build_system;
mod project;

mod global;
mod helpers;
mod ruff;

#[pyclass(frozen, get_all)]
pub struct Settings {
    column_width: usize,
    indent: usize,
    keep_full_version: bool,
    max_supported_python: (u8, u8),
    min_supported_python: (u8, u8),
}

#[pymethods]
impl Settings {
    #[new]
    #[pyo3(signature = (*, column_width, indent, keep_full_version, max_supported_python, min_supported_python ))]
    const fn new(
        column_width: usize,
        indent: usize,
        keep_full_version: bool,
        max_supported_python: (u8, u8),
        min_supported_python: (u8, u8),
    ) -> Self {
        Self {
            column_width,
            indent,
            keep_full_version,
            max_supported_python,
            min_supported_python,
        }
    }
}

/// Format toml file
#[must_use]
#[pyfunction]
pub fn format_toml(content: &str, opt: &Settings) -> String {
    let root_ast = parse(content).into_syntax().clone_for_update();
    let mut tables = Tables::from_ast(&root_ast);

    build_system::fix(&mut tables, opt.keep_full_version);
    project::fix(
        &mut tables,
        opt.keep_full_version,
        opt.max_supported_python,
        opt.min_supported_python,
    );
    ruff::fix(&mut tables);
    reorder_tables(&root_ast, &mut tables);

    let options = Options {
        align_entries: false,         // do not align by =
        align_comments: true,         // align inline comments
        align_single_comments: true,  // align comments after entries
        array_trailing_comma: true,   // ensure arrays finish with trailing comma
        array_auto_expand: true,      // arrays go to multi line when too long
        array_auto_collapse: false,   // do not collapse for easier diffs
        compact_arrays: false,        // leave whitespace
        compact_inline_tables: false, // leave whitespace
        compact_entries: false,       // leave whitespace
        column_width: opt.column_width,
        indent_tables: false,
        indent_entries: false,
        inline_table_expand: true,
        trailing_newline: true,
        allowed_blank_lines: 1, // one blank line to separate
        indent_string: " ".repeat(opt.indent),
        reorder_keys: false,   // respect custom order
        reorder_arrays: false, // for natural sorting we need to this ourselves
        crlf: false,
    };
    format_syntax(root_ast, options)
}

/// # Errors
///
/// Will return `PyErr` if an error is raised during formatting.
#[pymodule]
#[pyo3(name = "_lib")]
#[cfg(not(tarpaulin_include))]
pub fn _lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(format_toml, m)?)?;
    m.add_class::<Settings>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::path::{Path, PathBuf};

    use indoc::indoc;
    use rstest::{fixture, rstest};

    use crate::{format_toml, Settings};

    #[rstest]
    #[case::simple(
        indoc ! {r#"
    # comment
    a= "b"
    [project]
    name="alpha"
    dependencies=[" e >= 1.5.0"]
    [build-system]
    build-backend="backend"
    requires=[" c >= 1.5.0", "d == 2.0.0"]
    [tool.mypy]
    mk="mv"
    "#},
        indoc ! {r#"
    # comment
    a = "b"

    [build-system]
    build-backend = "backend"
    requires = [
      "c>=1.5",
      "d==2",
    ]

    [project]
    name = "alpha"
    classifiers = [
      "Programming Language :: Python :: 3 :: Only",
      "Programming Language :: Python :: 3.8",
      "Programming Language :: Python :: 3.9",
      "Programming Language :: Python :: 3.10",
      "Programming Language :: Python :: 3.11",
      "Programming Language :: Python :: 3.12",
    ]
    dependencies = [
      "e>=1.5",
    ]

    [tool.mypy]
    mk = "mv"
    "#},
        2,
        false,
        (3, 12),
    )]
    #[case::empty(
        indoc ! {r""},
        "\n",
        2,
        true,
        (3, 12)
    )]
    #[case::scripts(
        indoc ! {r#"
    [project.scripts]
    c = "d"
    a = "b"
    "#},
        indoc ! {r#"
    [project]
    classifiers = [
      "Programming Language :: Python :: 3 :: Only",
      "Programming Language :: Python :: 3.8",
    ]
    scripts.a = "b"
    scripts.c = "d"
    "#},
        2,
        true,
        (3, 8)
    )]
    #[case::subsubtable(
        indoc ! {r"
    [project]
    [tool.coverage.report]
    a = 2
    [tool.coverage]
    a = 0
    [tool.coverage.paths]
    a = 1
    [tool.coverage.run]
    a = 3
    "},
        indoc ! {r#"
    [project]
    classifiers = [
      "Programming Language :: Python :: 3 :: Only",
      "Programming Language :: Python :: 3.8",
    ]

    [tool.coverage]
    a = 0
    [tool.coverage.report]
    a = 2
    [tool.coverage.paths]
    a = 1
    [tool.coverage.run]
    a = 3
    "#},
        2,
        true,
        (3, 8)
    )]
    #[case::array_of_tables(
        indoc ! {r#"
        [tool.commitizen]
        name = "cz_customize"

        [tool.commitizen.customize]
        message_template = ""

        [[tool.commitizen.customize.questions]]
        type = "list"
        [[tool.commitizen.customize.questions]]
        type = "input"
    "#},
        indoc ! {r#"
    [tool.commitizen]
    name = "cz_customize"

    [tool.commitizen.customize]
    message_template = ""

    [[tool.commitizen.customize.questions]]
    type = "list"

    [[tool.commitizen.customize.questions]]
    type = "input"
    "#},
        2,
        true,
        (3, 8)
    )]
    #[case::unstable_issue_18(
        indoc ! {r#"
    [project]
    requires-python = "==3.12"
    classifiers = [
        "Programming Language :: Python :: 3 :: Only",
        "Programming Language :: Python :: 3.12",
    ]
    [project.urls]
    Source = "https://github.com/VWS-Python/vws-python-mock"

    [tool.setuptools]
    zip-safe = false
    "#},
        indoc ! {r#"
    [project]
    requires-python = "==3.12"
    classifiers = [
      "Programming Language :: Python :: 3 :: Only",
      "Programming Language :: Python :: 3.12",
    ]
    urls.Source = "https://github.com/VWS-Python/vws-python-mock"

    [tool.setuptools]
    zip-safe = false
    "#},
        2,
        true,
        (3, 8)
    )]
    fn test_format_toml(
        #[case] start: &str,
        #[case] expected: &str,
        #[case] indent: usize,
        #[case] keep_full_version: bool,
        #[case] max_supported_python: (u8, u8),
    ) {
        let settings = Settings {
            column_width: 1,
            indent,
            keep_full_version,
            max_supported_python,
            min_supported_python: (3, 8),
        };
        let got = format_toml(start, &settings);
        assert_eq!(got, expected);
        let second = format_toml(got.as_str(), &settings);
        assert_eq!(second, got);
    }

    #[fixture]
    fn data() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("rust")
            .join("src")
            .join("data")
    }

    #[rstest]
    fn test_issue_24(data: PathBuf) {
        let start = read_to_string(data.join("ruff-order.start.toml")).unwrap();
        let settings = Settings {
            column_width: 1,
            indent: 2,
            keep_full_version: false,
            max_supported_python: (3, 8),
            min_supported_python: (3, 8),
        };
        let got = format_toml(start.as_str(), &settings);
        let expected = read_to_string(data.join("ruff-order.expected.toml")).unwrap();
        assert_eq!(got, expected);
        let second = format_toml(got.as_str(), &settings);
        assert_eq!(second, got);
    }

    /// Test that the column width is respected,
    /// and that arrays are neither exploded nor collapsed without reason
    #[rstest]
    fn test_column_width() {
        let start = indoc! {r#"
        [build-system]
        build-backend = "backend"
        requires = ["c>=1.5", "d == 2" ]

        [project]
        name = "beta"
        dependencies = [
        "e>=1.5",
        ]
        "#};
        let settings = Settings {
            column_width: 80,
            indent: 4,
            keep_full_version: false,
            max_supported_python: (3, 12),
            min_supported_python: (3, 12),
        };
        let got = format_toml(start, &settings);
        let expected = indoc! {r#"
        [build-system]
        build-backend = "backend"
        requires = [ "c>=1.5", "d==2" ]

        [project]
        name = "beta"
        classifiers = [
            "Programming Language :: Python :: 3 :: Only",
            "Programming Language :: Python :: 3.12",
        ]
        dependencies = [
            "e>=1.5",
        ]
        "#};
        assert_eq!(got, expected);
        let second = format_toml(got.as_str(), &settings);
        assert_eq!(second, got);
    }
}
