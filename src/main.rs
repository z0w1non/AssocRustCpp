use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::{TempDir, tempdir};
use walkdir::WalkDir;

const DEFUALT_TEST_DIRNAME: &str = "test";
const CPP_SOURCE_FILENAME: &str = "main.cpp";
const CPP_EXE_FILENAME: &str = "cpp_test";
const RS_SOURCE_FILENAME: &str = "main.rs";
const RS_EXE_FILENAME: &str = "rs_test";

// C++の単体テストを実行するクラス
struct CppTestRunner {
    dir_path: PathBuf,
    source_path: PathBuf,
    _temp_dir: TempDir,
}

impl CppTestRunner {
    fn new(source_code: &str) -> Result<Self, Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path().to_path_buf();
        let source_path = dir_path.join(CPP_SOURCE_FILENAME);
        File::create(&source_path)?.write_all(source_code.as_bytes())?;

        Ok(Self {
            dir_path: dir_path,
            source_path: source_path,
            _temp_dir: temp_dir,
        })
    }

    fn run(&self) -> Result<String, Box<dyn Error>> {
        let cpp_exe = self.dir_path.join(CPP_EXE_FILENAME);

        // https://learn.microsoft.com/ja-jp/cpp/build/building-on-the-command-line?view=msvc-170
        let vcvars_candidates = [
            r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat",
            r"C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat",
            r"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvars64.bat",
            r"C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat",
        ];

        let vcvars_path = vcvars_candidates
            .iter()
            .find(|path| Path::new(path).exists())
            .ok_or("")?;

        let command = format!(
            r#"""{}" && cl /EHsc /std:c++17 /utf-8 "{}" /Fe:"{}"""#,
            vcvars_path,
            self.source_path
                .to_str()
                .ok_or("File path contains invalid UTF-8 strings")?,
            cpp_exe
                .to_str()
                .ok_or("File path contains invalid UTF-8 strings")?
        );

        let cpp_compile = Command::new("cmd").arg("/c").raw_arg(&command).output()?;

        if !cpp_compile.status.success() {
            return Err(format!(
                "C++ compile error:\n{}",
                String::from_utf8_lossy(&cpp_compile.stderr)
            )
            .into());
        }

        let output = Command::new(&cpp_exe).output()?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

// Rustの単体テストを実行するクラス
struct RustTestRunner {
    dir_path: PathBuf,
    source_path: PathBuf,
    _temp_dir: TempDir,
}

impl RustTestRunner {
    fn new(source_code: &str) -> Result<Self, Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path().to_path_buf();
        let source_path = dir_path.join(RS_SOURCE_FILENAME);
        File::create(&source_path)?.write_all(source_code.as_bytes())?;

        Ok(Self {
            dir_path: dir_path,
            source_path: source_path,
            _temp_dir: temp_dir,
        })
    }

    fn run(&self) -> Result<String, Box<dyn Error>> {
        let rs_exe = self.dir_path.join(RS_EXE_FILENAME);
        let rs_compile = Command::new("rustc")
            .args(&[
                self.source_path.to_str().ok_or("")?,
                "-o",
                rs_exe.to_str().ok_or("")?,
            ])
            .output()?;

        if !rs_compile.status.success() {
            return Err(format!(
                "Rust compile error:\n{}",
                String::from_utf8_lossy(&rs_compile.stderr)
            )
            .into());
        }

        let output = Command::new(&rs_exe).output()?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

// マークダウン形式でRustとC++の単体テストを定義したファイルをパースする
fn parse_test_file(file_path: &str) -> Result<(String, String, String), Box<dyn Error>> {
    let test_name = Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("File path contains invalid UTF-8 strings")?;

    let input_markdown = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("File open failed. '{}': {}", file_path, e).into()),
    };

    let mut cpp_code = String::new();
    let mut rs_code = String::new();

    let parser = Parser::new(&input_markdown);
    let mut current_lang = None;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                current_lang = Some(lang.to_string());
            }
            Event::Text(text) => match current_lang.as_deref() {
                Some("cpp") | Some("c++") => cpp_code.push_str(&text),
                Some("rs") | Some("rust") => rs_code.push_str(&text),
                _ => {}
            },
            Event::End(TagEnd::CodeBlock) => {
                current_lang = None;
            }
            _ => {}
        }
    }

    if cpp_code.is_empty() || rs_code.is_empty() {
        return Err("Required code blocks (cpp/rs) not found in the markdown file.".into());
    }

    Ok((test_name.to_string(), cpp_code, rs_code))
}

// C++とRustのコードブロックを含むマークダウン形式のファイルを元に、
// C++とRustをそれぞれビルド・実行し、その実行結果が同一となるか検証する。
fn test_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let file_path_str = path
        .to_str()
        .ok_or("File path contains invalid UTF-8 strings")?;
    let (test_name, cpp_code, rs_code) = parse_test_file(file_path_str)?;

    let cpp_test_runner = CppTestRunner::new(&cpp_code)?;
    let rust_test_runner = RustTestRunner::new(&rs_code)?;

    let cpp_output = match cpp_test_runner.run() {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Test {} [SKIP  ] C++ execution failed: {}", test_name, e);
            return Err(format!("Test {} [SKIP  ] C++ execution failed: {}", test_name, e).into());
        }
    };

    let rs_output = match rust_test_runner.run() {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Test {} [SKIP  ] Rust execution failed: {}", test_name, e);
            return Err(format!("Test {} [SKIP  ] C++ execution failed: {}", test_name, e).into());
        }
    };

    let cpp_stdout_trimmed = cpp_output.trim();
    let rs_stdout_trimmed = rs_output.trim();

    if cpp_stdout_trimmed == rs_stdout_trimmed {
        println!(
            "Test {} [PASSED]\nOutput: {}",
            test_name, cpp_stdout_trimmed
        );
    } else {
        println!(
            "Test {} [FAILED]\nRust Output: {}\nC++ Output: {}",
            test_name, rs_stdout_trimmed, cpp_stdout_trimmed
        );
    }

    Ok(())
}

// 再帰的にサブディレクトリに対してtest_fileを実行する。
fn test_dir(test_dir: &Path) -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new(test_dir).sort_by_file_name() {
        let entry = entry.map_err(|e| format!("directory seaching failed: {}", e))?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
            test_file(path)?;
        }
    }

    Ok(())
}

fn parse_command_line() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let target = if args.len() > 1 {
        &args[1]
    } else {
        DEFUALT_TEST_DIRNAME
    };

    let target_path: &Path = Path::new(target);
    if target_path.is_dir() {
        test_dir(&target_path)?;
    } else {
        let target_file_path: PathBuf = PathBuf::from(format!("{}.txt", target));
        if target_file_path.is_file() {
            test_file(&target_file_path)?;
        }
    }

    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    parse_command_line()?;
    
    Ok(())
}
