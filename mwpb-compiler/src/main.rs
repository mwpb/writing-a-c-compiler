mod ast;
mod tokens;
use clap::Parser;
use std::{fs, os::unix::process::CommandExt, path::Path, process::Command};
mod assembly;

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
    let raw = fs::read_to_string(preprocessed_path)?;
    let tokens = tokens::tokenize(&raw)?;
    let (programme, _) = ast::parse_programme(&*tokens)?;
    let assembly = assembly::generate_programme(programme);
    let assembly_text = assembly::print_programme(assembly);
    fs::write(assembly_path, assembly_text)?;
    Ok(())
}

fn assemble_and_link(assembly_path: &Path, executable_path: &Path) -> anyhow::Result<()> {
    Command::new("gcc")
        .arg(assembly_path.as_os_str())
        .arg("-o")
        .arg(executable_path.as_os_str())
        .exec();
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    let source_path = Path::new(&args.source_path);
    let preprocessed_path = source_path.with_extension("i");
    let assembly_path = source_path.with_extension("s");
    let executable_path = source_path.with_extension("");

    preprocess(source_path, &preprocessed_path).unwrap();
    compile(&preprocessed_path, &assembly_path).unwrap();
    fs::remove_file(preprocessed_path).unwrap();
    assemble_and_link(&assembly_path, &executable_path).unwrap();
    fs::remove_file(assembly_path).unwrap();

    Ok(())
}
