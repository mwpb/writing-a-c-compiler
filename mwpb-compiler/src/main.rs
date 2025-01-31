mod args;
mod ast;
mod tokens;
use args::Stage;
use clap::Parser;
use std::{fs, os::unix::process::CommandExt, path::Path, process::Command};
mod assembly;

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

fn compile(
    preprocessed_path: &Path,
    assembly_path: &Path,
    executable_path: &Path,
    stage: Stage,
) -> anyhow::Result<()> {
    let raw = fs::read_to_string(preprocessed_path)?;
    let tokens = tokens::tokenize(&raw)?;
    if stage == Stage::Lex {
        return Ok(());
    }

    let (programme, _) = ast::parse_programme(&*tokens)?;
    if stage == Stage::Parse {
        return Ok(());
    }

    let assembly = assembly::generate_programme(programme);
    if stage == Stage::CodeGen {
        return Ok(());
    }

    let assembly_text = assembly::print_programme(assembly);
    fs::write(assembly_path, assembly_text)?;
    Command::new("gcc")
        .arg(assembly_path.as_os_str())
        .arg("-o")
        .arg(executable_path.as_os_str())
        .exec();
    Ok(())
}

fn clean_intermediate_files(preprocessed_path: &Path, assembly_path: &Path) {
    let _ = fs::remove_file(preprocessed_path);
    let _ = fs::remove_file(assembly_path);
}

fn main() -> anyhow::Result<()> {
    let args = args::CliArgs::parse();
    let source_path = Path::new(&args.source_path);
    let stage: Stage = args::stage_from_args(&args);

    let preprocessed_path = source_path.with_extension("i");
    let assembly_path = source_path.with_extension("s");
    let executable_path = source_path.with_extension("");

    preprocess(source_path, &preprocessed_path).unwrap();
    compile(&preprocessed_path, &assembly_path, &executable_path, stage).unwrap();
    clean_intermediate_files(&preprocessed_path, &assembly_path);

    Ok(())
}
