use clap::Parser;
use std::process::{Command, ExitStatus};

fn run_preprocessor(input_file: &str, output_file: &str) -> std::io::Result<ExitStatus> {
    Command::new("gcc")
        .args(&["-E", "-P", input_file, "-o", output_file])
        .spawn()
        .expect("Failed to run preprocessor")
        .wait()
}

fn run_compiler(input_file: &str, output_file: &str) {
    Command::new("cp")
        .args(&[input_file, output_file])
        .spawn()
        .expect("Failed to run linker")
        .wait()
        .unwrap();
}

fn run_linker(input_file: &str, output_file: &str) -> std::io::Result<ExitStatus> {
    Command::new("gcc")
        .args(&[input_file, "-o", output_file])
        .spawn()
        .expect("Failed to run linker")
        .wait()
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    filename: String,

    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    codegen: bool,
}

fn main() {
    let cli = Cli::parse();

    let c_file = cli.filename;
    let i_file = format!("{}i", &c_file[..c_file.len() - 1]);
    let s_file = format!("{}s", &c_file[..c_file.len() - 1]);
    let bin = format!("{}", &c_file[..c_file.len() - 1]);

    run_preprocessor(&c_file, &i_file).expect("Error during preprocessing");
    run_compiler(&i_file, &s_file);
    run_linker(&s_file, &bin).expect("Error during linking");
}
