from __future__ import annotations

from textwrap import dedent

from pyproject_fmt_rust import Settings, format_toml


def test_format_toml() -> None:
    txt = """
    [project]
    keywords = [ "A" ]
    classifiers = [
      "Programming Language :: Python :: 3 :: Only",
    ]
    dynamic = [ "B" ]
    dependencies = [
      "requests>=2.0",
    ]
    """

    settings = Settings(
        column_width=120,
        indent=4,
        keep_full_version=True,
        min_supported_python=(3, 7),
        max_supported_python=(3, 8),
    )
    res = format_toml(dedent(txt), settings)

    expected = """\
    [project]
    keywords = [ "A" ]
    classifiers = [
        "Programming Language :: Python :: 3 :: Only",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
    ]
    dynamic = [ "B" ]
    dependencies = [
        "requests>=2.0",
    ]
    """
    assert res == dedent(expected)
