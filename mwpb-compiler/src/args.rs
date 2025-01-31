use clap::Parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Stage {
    Lex,
    Parse,
    CodeGen,
    Compile,
}

#[derive(Parser, Debug)]
pub struct CliArgs {
    pub source_path: String,

    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    codegen: bool,
}

pub fn stage_from_args(cli_args: &CliArgs) -> Stage {
    if cli_args.codegen {
        return Stage::CodeGen;
    }

    if cli_args.parse {
        return Stage::Parse;
    }

    if cli_args.lex {
        return Stage::Lex;
    }

    Stage::Compile
}
