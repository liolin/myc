use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
    process::{exit, Command, ExitStatus},
};

fn run_preprocessor(input_file: &str, output_file: &str) -> std::io::Result<ExitStatus> {
    Command::new("gcc")
        .args(&["-E", "-P", input_file, "-o", output_file])
        .spawn()
        .expect("Failed to run preprocessor")
        .wait()
}

fn run_compiler(input_file: &str, output_file: &str, args: &Cli) {
    let mut file = File::open(input_file).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();

    let mut token_stream = myc::lex(&source);

    if args.lex {
        let lexed_succesfully = token_stream.all(|t| !matches!(t, myc::lexer::Token::Invalid(_)));
        if !lexed_succesfully {
            eprintln!("Lex Error: Found an invalid token");
            exit(1);
        }

        return;
    }

    let ast = myc::parse(token_stream);

    if ast.is_err() {
        eprintln!("Parse error: {}", ast.unwrap_err());
        exit(1);
    }

    if args.parse {
        return;
    }

    let tacky = myc::tacky(ast.expect("already checked previousley"));

    if args.tacky {
        return;
    }

    let assembly = myc::assembly(tacky);

    if args.codegen {
        return;
    }

    let code = myc::codegen(assembly);
    let mut file = std::fs::File::create(output_file).unwrap();
    writeln!(&mut file, "{code}").unwrap();
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
    tacky: bool,

    #[arg(long)]
    codegen: bool,
}

fn main() {
    let cli = Cli::parse();

    let c_file = cli.filename.clone();
    let i_file = format!("{}i", &c_file[..c_file.len() - 1]);
    let s_file = format!("{}s", &c_file[..c_file.len() - 1]);
    let bin = format!("{}", &c_file[..c_file.len() - 2]);

    run_preprocessor(&c_file, &i_file).expect("Error during preprocessing");
    run_compiler(&i_file, &s_file, &cli);
    run_linker(&s_file, &bin).expect("Error during linking");
}
