use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;


#[derive(Debug, Clone)]
struct ParsedDoc {
    version: String,
    tasks: Vec<String>,
    workflows: Vec<String>,
}

#[derive(Debug, Clone)]
struct LintDiag {
    severity: String,
    message: String,
}


fn parse_wdl(source: &str) -> Result<ParsedDoc, String> {
    let mut version = None;
    let mut tasks = vec![];
    let mut workflows = vec![];

    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("version ") {
            let v = trimmed.strip_prefix("version ").unwrap().trim().to_string();
            if v.is_empty() {
                return Err(format!("Line {}: empty version string", i + 1));
            }
            version = Some(v);
        } else if trimmed.starts_with("task ") {
            let rest = trimmed.strip_prefix("task ").unwrap();
            let name = rest.split_whitespace().next().unwrap_or("").to_string();
            if name.is_empty() {
                return Err(format!("Line {}: task has no name", i + 1));
            }
            tasks.push(name);
        } else if trimmed.starts_with("workflow ") {
            let rest = trimmed.strip_prefix("workflow ").unwrap();
            let name = rest.split_whitespace().next().unwrap_or("").to_string();
            if name.is_empty() {
                return Err(format!("Line {}: workflow has no name", i + 1));
            }
            workflows.push(name);
        } else if !trimmed.is_empty()
            && !trimmed.starts_with('#')
            && !trimmed.starts_with('}')
            && !trimmed.starts_with('{')
            && !trimmed.starts_with("command")
            && !trimmed.starts_with("meta")
            && !trimmed.starts_with("parameter_meta")
            && version.is_none()
        {
            return Err(format!(
                "Line {}: unexpected token '{}' — did you forget 'version 1.x'?",
                i + 1,
                trimmed
            ));
        }
    }

    let version = version.ok_or("Missing 'version' declaration")?;
    Ok(ParsedDoc { version, tasks, workflows })
}

fn lint_doc(doc: &ParsedDoc) -> Vec<LintDiag> {
    let mut diags = vec![];

    let known = ["1.0", "1.1", "1.2"];
    if !known.contains(&doc.version.as_str()) {
        diags.push(LintDiag {
            severity: "warning".into(),
            message: format!(
                "Unknown WDL version '{}'. Expected one of: 1.0, 1.1, 1.2",
                doc.version
            ),
        });
    }

    for task in &doc.tasks {
        if task.chars().any(|c| c.is_uppercase()) {
            diags.push(LintDiag {
                severity: "warning".into(),
                message: format!(
                    "Task '{}' should use snake_case (e.g. '{}')",
                    task,
                    to_snake_case(task)
                ),
            });
        }
    }

    for wf in &doc.workflows {
        if wf.chars().any(|c| c.is_uppercase()) {
            diags.push(LintDiag {
                severity: "warning".into(),
                message: format!(
                    "Workflow '{}' should use snake_case (e.g. '{}')",
                    wf,
                    to_snake_case(wf)
                ),
            });
        }
    }

    if doc.tasks.is_empty() && doc.workflows.is_empty() {
        diags.push(LintDiag {
            severity: "info".into(),
            message: "Document has no tasks or workflows defined.".into(),
        });
    }

    diags
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i != 0 {
            out.push('_');
        }
        out.push(c.to_lowercase().next().unwrap());
    }
    out
}


#[pyclass(name = "Document")]
#[derive(Clone)]
struct PyDocument {
    inner: ParsedDoc,
}

#[pymethods]
impl PyDocument {
    #[getter]
    fn version(&self) -> &str {
        &self.inner.version
    }

    #[getter]
    fn task_names(&self) -> Vec<String> {
        self.inner.tasks.clone()
    }

    #[getter]
    fn workflow_names(&self) -> Vec<String> {
        self.inner.workflows.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Document(version={:?}, tasks={:?}, workflows={:?})",
            self.inner.version, self.inner.tasks, self.inner.workflows
        )
    }
}

#[pyclass(name = "Diagnostic")]
#[derive(Clone)]
struct PyDiagnostic {
    inner: LintDiag,
}

#[pymethods]
impl PyDiagnostic {
    #[getter]
    fn severity(&self) -> &str {
        &self.inner.severity
    }

    #[getter]
    fn message(&self) -> &str {
        &self.inner.message
    }

    fn __repr__(&self) -> String {
        format!("Diagnostic({}: {})", self.inner.severity, self.inner.message)
    }
}
#[pyfunction]
fn parse(source: &str) -> PyResult<PyDocument> {
    parse_wdl(source)
        .map(|inner| PyDocument { inner })
        .map_err(|e| PyValueError::new_err(format!("ParseError: {}", e)))
}

#[pyfunction]
fn lint(doc: &PyDocument) -> Vec<PyDiagnostic> {
    lint_doc(&doc.inner)
        .into_iter()
        .map(|inner| PyDiagnostic { inner })
        .collect()
}


#[pymodule]
fn wdl_lite(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDocument>()?;
    m.add_class::<PyDiagnostic>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(lint, m)?)?;
    m.add("ParseError", m.py().get_type::<PyValueError>())?;
    Ok(())
}
