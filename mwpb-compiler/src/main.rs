mod tokens;
use clap::Parser;
use std::{fs, path::Path, process::Command};

#[derive(Parser, Debug)]
struct CliArgs {
    source_path: String,

    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    codegen: bool,
}

fn preprocess(source_path: &Path, preprocessed_file: &Path) -> anyhow::Result<()> {
    Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(source_path.as_os_str())
        .arg("-o")
        .arg(preprocessed_file.as_os_str())
        .output()?;
    Ok(())
}

fn compile(preprocessed_path: &Path, assembly_path: &Path) -> anyhow::Result<()> {
    let programme = fs::read_to_string(preprocessed_path)?;
    let tokens = tokens::tokenize(&programme)?;
    fs::write(assembly_path, programme)?;
    Ok(())
}

fn assemble_and_link(assembly_path: &Path, executable_path: &Path) -> anyhow::Result<()> {
    Command::new("gcc")
        .arg(assembly_path.as_os_str())
        .arg("-o")
        .arg(executable_path.as_os_str())
        .output()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    let source_path = Path::new(&args.source_path);
    let preprocessed_path = source_path.with_extension("i");
    let assembly_path = source_path.with_extension("s");
    let executable_path = source_path.with_extension("");

    preprocess(source_path, &preprocessed_path)?;
    compile(&preprocessed_path, &assembly_path)?;
    fs::remove_file(preprocessed_path)?;
    assemble_and_link(&assembly_path, &executable_path)?;
    fs::remove_file(assembly_path)?;

    Ok(())
}
