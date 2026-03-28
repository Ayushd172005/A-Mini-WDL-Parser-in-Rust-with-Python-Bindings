import pytest
import wdl_lite

def test_parse_basic():
    doc = wdl_lite.parse("version 1.2\ntask hello { command { echo hi } }")
    assert doc.version == "1.2"
    assert doc.task_names == ["hello"]

def test_parse_error():
    with pytest.raises(ValueError, match="ParseError"):
        wdl_lite.parse("this is not wdl")

def test_lint_snake_case():
    doc = wdl_lite.parse("version 1.2\ntask MyTask { command {} }")
    diags = wdl_lite.lint(doc)
    assert any("snake_case" in d.message for d in diags)

def test_lint_unknown_version():
    doc = wdl_lite.parse("version 9.9\ntask t { command {} }")
    diags = wdl_lite.lint(doc)
    assert any("Unknown WDL version" in d.message for d in diags)

def test_empty_document():
    doc = wdl_lite.parse("version 1.1")
    diags = wdl_lite.lint(doc)
    assert any("no tasks or workflows" in d.message for d in diags)
