use crate::helpers::array::{sort, transform};
use crate::helpers::string::update_content;
use crate::helpers::table::{collapse_sub_tables, for_entries, reorder_table_keys, Tables};

#[allow(clippy::too_many_lines)]
pub fn fix(tables: &mut Tables) {
    collapse_sub_tables(tables, "tool.ruff");
    let table_element = tables.get("tool.ruff");
    if table_element.is_none() {
        return;
    }
    let table = &mut table_element.unwrap().first().unwrap().borrow_mut();
    for_entries(table, &mut |key, entry| match key.as_str() {
        "target-version"
        | "cache-dir"
        | "extend"
        | "required-version"
        | "output-format"
        | "format.indent-style"
        | "format.line-ending"
        | "format.quote-style"
        | "lint.dummy-variable-rgx"
        | "lint.flake8-copyright.author"
        | "lint.flake8-copyright.notice-rgx"
        | "lint.flake8-pytest-style.parametrize-names-type"
        | "lint.flake8-pytest-style.parametrize-values-row-type"
        | "lint.flake8-pytest-style.parametrize-values-type"
        | "lint.flake8-quotes.docstring-quotes"
        | "lint.flake8-quotes.multiline-quotes"
        | "lint.flake8-quotes.inline-quotes"
        | "lint.flake8-tidy-imports.ban-relative-imports"
        | "lint.isort.known-first-party"
        | "lint.isort.known-third-party"
        | "lint.isort.relative-imports-order"
        | "lint.pydocstyle.convention" => {
            update_content(entry, |s| String::from(s));
        }
        "exclude"
        | "extend-exclude"
        | "builtins"
        | "include"
        | "extend-include"
        | "namespace-packages"
        | "src"
        | "format.exclude"
        | "lint.allowed-confusables"
        | "lint.exclude"
        | "lint.extend-fixable"
        | "lint.extend-ignore"
        | "lint.extend-safe-fixes"
        | "lint.extend-select"
        | "lint.extend-unsafe-fixes"
        | "lint.external"
        | "lint.fixable"
        | "lint.ignore"
        | "lint.logger-objects"
        | "lint.select"
        | "lint.task-tags"
        | "lint.typing-modules"
        | "lint.unfixable"
        | "lint.flake8-bandit.hardcoded-tmp-directory"
        | "lint.flake8-bandit.hardcoded-tmp-directory-extend"
        | "lint.flake8-boolean-trap.extend-allowed-calls"
        | "lint.flake8-bugbear.extend-immutable-calls"
        | "lint.flake8-builtins.builtins-ignorelist"
        | "lint.flake8-gettext.extend-function-names"
        | "lint.flake8-gettext.function-names"
        | "lint.flake8-import-conventions.banned-from"
        | "lint.flake8-pytest-style.raises-extend-require-match-for"
        | "lint.flake8-pytest-style.raises-require-match-for"
        | "lint.flake8-self.extend-ignore-names"
        | "lint.flake8-self.ignore-names"
        | "lint.flake8-tidy-imports.banned-module-level-imports"
        | "lint.flake8-type-checking.exempt-modules"
        | "lint.flake8-type-checking.runtime-evaluated-base-classes"
        | "lint.flake8-type-checking.runtime-evaluated-decorators"
        | "lint.isort.constants"
        | "lint.isort.default-section"
        | "lint.isort.extra-standard-library"
        | "lint.isort.forced-separate"
        | "lint.isort.no-lines-before"
        | "lint.isort.required-imports"
        | "lint.isort.single-line-exclusions"
        | "lint.isort.variables"
        | "lint.pep8-naming.classmethod-decorators"
        | "lint.pep8-naming.extend-ignore-names"
        | "lint.pep8-naming.ignore-names"
        | "lint.pep8-naming.staticmethod-decorators"
        | "lint.pydocstyle.ignore-decorators"
        | "lint.pydocstyle.property-decorators"
        | "lint.pyflakes.extend-generics"
        | "lint.pylint.allow-dunder-method-names"
        | "lint.pylint.allow-magic-value-types" => {
            transform(entry, &|s| String::from(s));
            sort(entry, str::to_lowercase);
        }
        "lint.isort.section-order" => {
            transform(entry, &|s| String::from(s));
        }
        _ => {
            if key.starts_with("lint.extend-per-file-ignores.") || key.starts_with("lint.per-file-ignores.") {
                transform(entry, &|s| String::from(s));
                sort(entry, str::to_lowercase);
            }
        }
    });
    reorder_table_keys(
        table,
        &[
            "",
            "required-version",
            "extend",
            "target-version",
            "line-length",
            "indent-width",
            "tab-size",
            "builtins",
            "namespace-packages",
            "src",
            "include",
            "extend-include",
            "exclude",
            "extend-exclude",
            "force-exclude",
            "respect-gitignore",
            "preview",
            "fix",
            "unsafe-fixes",
            "fix-only",
            "show-fixes",
            "show-source",
            "output-format",
            "cache-dir",
            "format.preview",
            "format.indent-style",
            "format.quote-style",
            "format.line-ending",
            "format.skip-magic-trailing-comma",
            "format.docstring-code-line-length",
            "format.docstring-code-format ",
            "format.exclude",
            "format",
            "lint.select",
            "lint.extend-select",
            "lint.ignore",
            "lint.explicit-preview-rules",
            "lint.exclude",
            "lint.extend-ignore",
            "lint.per-file-ignores",
            "lint.extend-per-file-ignores",
            "lint.fixable",
            "lint.extend-fixable",
            "lint.unfixable",
            "lint.extend-safe-fixes",
            "lint.extend-unsafe-fixes",
            "lint.typing-modules",
            "lint.allowed-confusables",
            "lint.dummy-variable-rgx",
            "lint.external",
            "lint.task-tags",
            "lint.flake8-annotations",
            "lint.flake8-bandit",
            "lint.flake8-boolean-trap",
            "lint.flake8-bugbear",
            "lint.flake8-builtins",
            "lint.flake8-comprehensions",
            "lint.flake8-copyright",
            "lint.flake8-errmsg",
            "lint.flake8-gettext",
            "lint.flake8-implicit-str-concat",
            "lint.flake8-import-conventions",
            "lint.flake8-pytest-style",
            "lint.flake8-quotes",
            "lint.flake8-self",
            "lint.flake8-tidy-imports",
            "lint.flake8-type-checking",
            "lint.flake8-unused-arguments",
            "lint.isort",
            "lint.mccabe",
            "lint.pep8-naming",
            "lint.pycodestyle",
            "lint.pydocstyle",
            "lint.pyflakes",
            "lint.pylint",
            "lint.pyupgrade",
            "lint",
        ],
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::path::{Path, PathBuf};

    use rstest::{fixture, rstest};
    use taplo::formatter::{format_syntax, Options};
    use taplo::parser::parse;
    use taplo::syntax::SyntaxElement;

    use crate::helpers::table::Tables;
    use crate::ruff::fix;

    fn evaluate(start: &str) -> String {
        let root_ast = parse(start).into_syntax().clone_for_update();
        let count = root_ast.children_with_tokens().count();
        let mut tables = Tables::from_ast(&root_ast);
        fix(&mut tables);
        let entries = tables
            .table_set
            .iter()
            .flat_map(|e| e.borrow().clone())
            .collect::<Vec<SyntaxElement>>();
        root_ast.splice_children(0..count, entries);
        let opt = Options {
            column_width: 1,
            ..Options::default()
        };
        format_syntax(root_ast, opt)
    }
    #[fixture]
    fn data() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("rust")
            .join("src")
            .join("data")
    }

    #[rstest]
    fn test_order_ruff(data: PathBuf) {
        let start = read_to_string(data.join("ruff-order.start.toml")).unwrap();
        let got = evaluate(start.as_str());
        let expected = read_to_string(data.join("ruff-order.expected.toml")).unwrap();
        assert_eq!(got, expected);
    }

    #[rstest]
    fn test_ruff_comment_21(data: PathBuf) {
        let start = read_to_string(data.join("ruff-21.start.toml")).unwrap();
        let got = evaluate(start.as_str());
        let expected = read_to_string(data.join("ruff-21.expected.toml")).unwrap();
        assert_eq!(got, expected);
    }
}
