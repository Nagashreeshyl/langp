//! Shared Lang.P CLI logic for `langc` and `lang`.

use langp_ast::to_json;
use langp_interpreter::run as interpret;
use langp_lexer::{format_tokens, lex};
use langp_parser::parse;
use langp_semantic::{analyze, Severity};
use std::fs;
use std::path::{Path, PathBuf};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliFlavor {
    Langc,
    Lang,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Parse,
    Check,
    Run,
    Build,
}

pub fn run(args: &[String], flavor: CliFlavor) -> Result<(), String> {
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage(flavor);
        return Ok(());
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        match flavor {
            CliFlavor::Langc => println!("langc {VERSION}"),
            CliFlavor::Lang => println!("lang {VERSION}"),
        }
        return Ok(());
    }

    if flavor == CliFlavor::Lang {
        if let Some(cmd) = args.first().map(|s| s.as_str()) {
            return match cmd {
                "init" => langpm::cmd_init(args.get(1).map(|s| s.as_str())),
                "install" => langpm::cmd_install(args.get(1).map(|s| s.as_str())),
                "remove" => langpm::cmd_remove(
                    args.get(1)
                        .map(|s| s.as_str())
                        .ok_or("usage: lang remove <package>")?,
                ),
                "update" => langpm::cmd_update(args.get(1).map(|s| s.as_str())),
                "search" => langpm::cmd_search(
                    args.get(1)
                        .map(|s| s.as_str())
                        .ok_or("usage: lang search <query>")?,
                ),
                "publish" => langpm::cmd_publish(),
                "login" => langpm::cmd_login(),
                "doctor" => langpm::cmd_doctor(),
                "fmt" => langpm::cmd_fmt(),
                "test" => langpm::cmd_test(),
                "build" if !args.iter().any(|a| a.ends_with(".lp")) => langpm::cmd_build(),
                _ => run_file(args, flavor),
            };
        }
    }

    run_file(args, flavor)
}

fn run_file(args: &[String], flavor: CliFlavor) -> Result<(), String> {

    let mode = parse_mode(args, flavor);
    let emit_tokens = has_flag(args, "--emit", "tokens");
    let emit_ast = has_flag(args, "--emit", "ast") || args.iter().any(|a| a == "--emit=ast");
    let output: Option<PathBuf> = flag_value(args, "-o")
        .or_else(|| flag_value(args, "--output"))
        .map(PathBuf::from);

    let file_path = input_file(args, flavor)?;
    let path = PathBuf::from(&file_path);
    let source = fs::read_to_string(&path)
        .map_err(|e| format!("failed to read '{}': {e}", path.display()))?;

    if emit_tokens {
        if flavor == CliFlavor::Lang {
            return Err("--emit is only available via langc".into());
        }
        let tokens = lex(&source).map_err(|e| e.to_string())?;
        println!("{}", format_tokens(&tokens));
        return Ok(());
    }

    let program = parse(&source).map_err(|e| e.to_string())?;

    if emit_ast {
        if flavor == CliFlavor::Lang {
            return Err("--emit is only available via langc".into());
        }
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
            if flavor == CliFlavor::Lang {
                return Err("specify a command: lang run <file.lp>".into());
            }
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

fn parse_mode(args: &[String], flavor: CliFlavor) -> Mode {
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
    if flavor == CliFlavor::Lang {
        if args.iter().any(|a| a.ends_with(".lp")) {
            return Mode::Run;
        }
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

fn input_file(args: &[String], flavor: CliFlavor) -> Result<String, String> {
    let skip = |s: &str| -> bool {
        matches!(
            s,
            "tokens"
                | "ast"
                | "langc"
                | "lang"
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
        .find(|a| {
            if skip(a) {
                return false;
            }
            if flavor == CliFlavor::Lang {
                a.ends_with(".lp") || Path::new(a).extension().is_some()
            } else {
                a.ends_with(".lp")
            }
        })
        .cloned()
        .ok_or_else(|| {
            if flavor == CliFlavor::Lang {
                "no file specified — try: lang run examples/hello.lp".into()
            } else {
                "no .lp input file specified".into()
            }
        })
}

fn default_build_output(source: &Path) -> PathBuf {
    let stem = source
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("program");
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
where lang >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
  where langc >nul 2>nul
  if %ERRORLEVEL% NEQ 0 (
    echo lang not found. Install from https://github.com/Nagashreeshyl/langp/releases
    exit /b 1
  )
  langc --mode run "%LANGP_SOURCE%" %*
) else (
  lang run "%LANGP_SOURCE%" %*
)
"#,
        );
        fs::write(output.with_extension("bat"), script)
            .map_err(|e| format!("write failed: {e}"))?;
    }

    let runner = format!(
        r#"#!/bin/sh
# Lang.P bundle — generated by lang build
# Source: {abs_source}
set -e
if command -v lang >/dev/null 2>&1; then
  exec lang run "{abs_source}" "$@"
fi
if command -v langc >/dev/null 2>&1; then
  exec langc --mode run "{abs_source}" "$@"
fi
echo "lang not found. Install: curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh" >&2
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

    let _ = escaped_source;
    Ok(())
}

fn install_self() -> Result<(), String> {
    println!("Lang.P installer");
    println!();
    println!("One-line install (macOS / Linux):");
    println!("  curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh");
    println!();
    println!("Windows (PowerShell):");
    println!("  irm https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.ps1 | iex");
    Ok(())
}

fn print_usage(flavor: CliFlavor) {
    match flavor {
        CliFlavor::Lang => {
            eprintln!(
                r#"lang — Lang.P v{VERSION}

USAGE:
    lang run <file.lp>          Run a program
    lang <file.lp>              Same as lang run
    lang check <file.lp>        Check for errors
    lang init [name]            Create langp.toml project
    lang install [package]      Install dependencies
    lang remove <package>       Remove dependency
    lang update                 Update lock file
    lang search <query>         Search packages
    lang build                  Build project (lang check entry)
    lang test                   Check tests/*.lp
    lang fmt                    Format/check entry file
    lang doctor                 Verify toolchain
    lang --version              Show version

EXAMPLES:
    lang run examples/hello.lp
    lang init my-app
    lang install filesystem
"#
            );
        }
        CliFlavor::Langc => {
            eprintln!(
                r#"langc — Lang.P compiler v{VERSION}

USAGE:
    langc run <file.lp>
    langc check <file.lp>
    langc build <file.lp> [-o OUTPUT]

For everyday use, prefer: lang run <file.lp>

OPTIONS:
    --emit tokens     Dump token stream
    --emit ast        Dump AST as JSON
    -o, --output      Output path for build
    --version, -V     Print version
    --help, -h        Print help
"#
            );
        }
    }
}
