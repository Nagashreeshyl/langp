//! langc — Lang.P compiler CLI

use langp_ast::to_json;
use langp_interpreter::run as interpret;
use langp_lexer::{format_tokens, lex};
use langp_parser::parse;
use langp_semantic::{analyze, Severity};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage();
        return Ok(());
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("langc {VERSION}");
        return Ok(());
    }

    if args.iter().any(|a| a == "install" || a == "--install") {
        return install_self();
    }

    let mode = parse_mode(&args);
    let emit_tokens = has_flag(&args, "--emit", "tokens");
    let emit_ast = has_flag(&args, "--emit", "ast") || args.iter().any(|a| a == "--emit=ast");
    let output: Option<PathBuf> = flag_value(&args, "-o")
        .or_else(|| flag_value(&args, "--output"))
        .map(PathBuf::from);

    let file_path = input_file(&args)?;
    let path = PathBuf::from(&file_path);
    let source = fs::read_to_string(&path)
        .map_err(|e| format!("failed to read '{}': {e}", path.display()))?;

    if emit_tokens {
        let tokens = lex(&source).map_err(|e| e.to_string())?;
        println!("{}", format_tokens(&tokens));
        return Ok(());
    }

    let program = parse(&source).map_err(|e| e.to_string())?;

    if emit_ast {
        println!("{}", to_json(&program).map_err(|e| e.to_string())?);
        return Ok(());
    }

    match mode {
        Mode::Check => {
            let result = analyze(&program);
            for d in &result.diagnostics {
                eprintln!("{d}");
            }
            if result.is_ok() {
                println!("✓ {} — no errors", path.display());
                Ok(())
            } else {
                Err(format!(
                    "check failed with {} error(s)",
                    result
                        .diagnostics
                        .iter()
                        .filter(|d| d.severity == Severity::Error)
                        .count()
                ))
            }
        }
        Mode::Run => {
            let check = analyze(&program);
            for d in &check.diagnostics {
                if d.severity == Severity::Error {
                    eprintln!("{d}");
                }
            }
            if !check.is_ok() {
                return Err("semantic errors prevent execution".into());
            }
            interpret(&program).map_err(|e| e.to_string())?;
            Ok(())
        }
        Mode::Build => {
            let check = analyze(&program);
            if !check.is_ok() {
                for d in &check.diagnostics {
                    eprintln!("{d}");
                }
                return Err("build failed: semantic errors".into());
            }
            let out = output.unwrap_or_else(|| default_build_output(&path));
            build_bundle(&path, &source, &out)?;
            println!("✓ built {}", out.display());
            Ok(())
        }
        Mode::Parse => {
            println!(
                "parsed {} top-level item(s) from {}",
                program.items.len(),
                path.display()
            );
            println!("(use --mode run, --mode check, or --emit ast)");
            Ok(())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Parse,
    Check,
    Run,
    Build,
}

fn parse_mode(args: &[String]) -> Mode {
    if let Some(i) = args.iter().position(|a| a == "--mode") {
        if let Some(m) = args.get(i + 1).map(|s| s.as_str()) {
            return match m {
                "check" => Mode::Check,
                "run" | "interpret" => Mode::Run,
                "build" | "compile" => Mode::Build,
                _ => Mode::Parse,
            };
        }
    }
    if args.iter().any(|a| a == "run" || a == "--run") {
        return Mode::Run;
    }
    if args.iter().any(|a| a == "check" || a == "--check") {
        return Mode::Check;
    }
    if args.iter().any(|a| a == "build" || a == "--build") {
        return Mode::Build;
    }
    Mode::Parse
}

fn has_flag(args: &[String], flag: &str, value: &str) -> bool {
    args.windows(2)
        .any(|w| w[0] == flag && w[1] == value)
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].clone())
}

fn input_file(args: &[String]) -> Result<String, String> {
    let skip = |s: &str| -> bool {
        matches!(
            s,
            "tokens"
                | "ast"
                | "langc"
                | "run"
                | "check"
                | "build"
                | "interpret"
                | "compile"
                | "install"
        ) || s.starts_with('-')
            || s.starts_with("--mode=")
    };

    args.iter()
        .rev()
        .find(|a| !skip(a) && a.ends_with(".lp"))
        .cloned()
        .ok_or_else(|| "no .lp input file specified".to_string())
}

fn default_build_output(source: &Path) -> PathBuf {
    let stem = source.file_stem().and_then(|s| s.to_str()).unwrap_or("program");
    if cfg!(windows) {
        PathBuf::from(format!("{stem}.exe"))
    } else {
        PathBuf::from(stem.to_string())
    }
}

fn build_bundle(source_path: &Path, source: &str, output: &Path) -> Result<(), String> {
    let abs_source = fs::canonicalize(source_path)
        .unwrap_or_else(|_| source_path.to_path_buf())
        .display()
        .to_string()
        .replace('\\', "\\\\")
        .replace('"', "\\\"");

    let escaped_source = source.replace('\\', "\\\\").replace('"', "\\\"");

    if cfg!(windows) {
        let script = format!(
            r#"@echo off
setlocal
set LANGP_SOURCE="{abs_source}"
where langc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
  echo langc not found. Install from https://github.com/Nagashreeshyl/langp/releases
  exit /b 1
)
langc --mode run "%LANGP_SOURCE%" %*
"#,
        );
        fs::write(output.with_extension("bat"), script)
            .map_err(|e| format!("write failed: {e}"))?;
    }

    let runner = format!(
        r#"#!/bin/sh
# Lang.P bundle — generated by langc build
# Source: {abs_source}
set -e
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
if command -v langc >/dev/null 2>&1; then
  exec langc --mode run "{abs_source}" "$@"
fi
# Fallback: embedded source (requires langc on PATH for full runtime)
if [ -f "$SCRIPT_DIR/.langp/source.lp" ]; then
  exec langc --mode run "$SCRIPT_DIR/.langp/source.lp" "$@"
fi
echo "langc not found. Install: curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh" >&2
exit 1
"#,
    );

    if !cfg!(windows) {
        fs::write(output, runner).map_err(|e| format!("write failed: {e}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(output)
                .map_err(|e| e.to_string())?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(output, perms).map_err(|e| e.to_string())?;
        }
    }

    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    let stem = output
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("program");
    let bundle_dir = parent.join(format!(".{stem}.langp"));
    fs::create_dir_all(&bundle_dir).map_err(|e| e.to_string())?;
    fs::write(bundle_dir.join("source.lp"), source).map_err(|e| e.to_string())?;
    fs::write(
        bundle_dir.join("meta.txt"),
        format!("source={abs_source}\nversion={VERSION}\n"),
    )
    .map_err(|e| e.to_string())?;

    let _ = escaped_source; // embedded copy kept in bundle dir
    Ok(())
}

fn install_self() -> Result<(), String> {
    println!("Lang.P installer");
    println!();
    println!("Quick install (macOS / Linux):");
    println!("  curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh");
    println!();
    println!("Windows (PowerShell):");
    println!("  irm https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.ps1 | iex");
    println!();
    println!("Or build from source (release, optimized):");
    println!("  cargo build --release -p langc");
    println!("  cp target/release/langc ~/.local/bin/");
    Ok(())
}

fn print_usage() {
    eprintln!(
        r#"langc — Lang.P compiler v{VERSION}

USAGE:
    langc [OPTIONS] <file.lp>
    langc run <file.lp>
    langc check <file.lp>
    langc build <file.lp> [-o OUTPUT]
    langc install

MODES:
    --mode run        Execute program (default with `langc run`)
    --mode check      Semantic analysis only
    --mode build      Build runnable bundle
    --mode interpret  Same as run

OPTIONS:
    --emit tokens     Dump token stream
    --emit ast        Dump AST as JSON
    -o, --output      Output path for build
    --version, -V     Print version
    --help, -h        Print help
    --install         Show install instructions

EXAMPLES:
    langc run examples/hello.lp
    langc check examples/input_demo.lp
    langc build examples/hello.lp -o hello
    langc --emit ast examples/hello.lp

INSTALL (other systems):
    curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh
"#
    );
}
