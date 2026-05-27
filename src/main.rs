/*
this is the main file, where the pipeline starts and ends
*/

//#![allow(unused)]

// rewrite
mod lexer;
mod parser;
mod ast;
mod backend;

use std::io::Write;
use std::time::Instant;
use std::fs;
use std::process;

use crate::lexer::TkType;

const HELP_MSG: &'static str = r#"
Options: 
    --help              Display this message
    --version           Print version info and exit
    -cc <CC>            Use CC to compile the C generated code (by default, clang is selected)
    -time-report        Prints a simple report 
    -o <FILENAME>       Write output to FILENAME

Debug Options: 
    --dbg-lexer-only    Only runs the lexer and prints the tokens, then exit
    --dbg_parsing_only  Only runs the lexer and parser and prints the tokens and AST, then exit
"#;

fn main() {
    // debug options
    let mut dbg_lex_only = false;
    let mut dbg_parsing_only = false;
    
    // compiler options
    let mut source_file = "";
    let mut output_file = "";
    let mut cc = "clang";
    let mut emit_c = false;
    let mut time_report = false;

    let args = std::env::args().collect::<Vec<String>>(); // we collect the command line arguments into a vector
    
    // handling the command line arguments
    let mut i: usize = 1;
    loop {
        if i >= args.len() { break; }
        let carg = args[i].as_str();
        match carg {
            "--dbg-lexer-only" => {
                dbg_lex_only = true;
            }
            "--dbg-parsing-only" => {
                dbg_parsing_only = true;
            }
            "-cc" => {
                i += 1;
                cc = args[i].as_str();
            }
            "-emit-c" => {
                emit_c = true;
            }
            "-time-report" => {
                time_report = true;
            }
            "--version" => {
                println!("parlan v0.2");
                return;
            }
            "--help" => {
                println!("usage: parlan [OPTIONS] INPUT");
                println!("{}", HELP_MSG);
                return;
            }
            "-o" => {
                i += 1;
                output_file = args[i].as_str();
            }
            file_name => {
                source_file = file_name
            }
        }
        i += 1;
    }

    let time_report_parsing_s;
    let time_report_parsing;
    let time_report_backend_s;
    let time_report_backend;

    let source = fs::read_to_string(source_file).unwrap();
    
    if dbg_lex_only {
        let mut lexer = lexer::Lexer::new(source.clone());
        loop {
            let tk = lexer.next_token();
            tk.print(&source);
            if tk.tk_type == TkType::Eof || tk.tk_type == TkType::Err { break; }
        }
        return;
    }

    let mut parser = ast::Parser::new(source.clone());
    time_report_parsing_s = Instant::now();
    parser.parse();
    time_report_parsing = time_report_parsing_s.elapsed();
    
    if dbg_parsing_only {
        parser.dbg_print();
        return;
    }

    let mut backend = backend::Backend::new(source);
    time_report_backend_s = Instant::now();
    let c = backend.emit_c(&parser);
    time_report_backend = time_report_backend_s.elapsed();

    if emit_c {
        let mut file = fs::File::create(output_file).unwrap();
        file.write(c.as_bytes()).expect("error: could not write to file");
        return;
    }

    let mut file = fs::File::create("__temp__parlan.c").unwrap();
    file.write(c.as_bytes()).expect("error: could not write to file");

    let cc_out = process::Command::new(cc)
                          .args(["-o",format!("{output_file}").as_str(), "__temp__parlan.c"])
                          .output()
                          .expect("[ERROR]: error while calling the C compiler.\n");
    if !process::ExitStatus::success(&cc_out.status) {
        eprintln!("{cc} output (stderr):");
        eprintln!("{}", String::from_utf8_lossy(&cc_out.stderr));
        eprintln!("[ERROR] could not finish compilation");
    }                    

    fs::remove_file("__temp__parlan.c").expect("error: could not remove temporal file"); // eliminate the temporal file

    // show time report (if requested)
    if time_report {
        eprintln!(r#"
===-------------------------------------------------------------------------===
                              Parlan Time Report
===-------------------------------------------------------------------------===
    Total Execution Time: {:.5} seconds

    --- Time ---        --- Name ---
    {:.5} sec           front end
    {:.5} sec           backend
"#, 
        time_report_backend.as_secs_f64() + time_report_parsing.as_secs_f64(),
        time_report_parsing.as_secs_f64(),
        time_report_backend.as_secs_f64());
    }
}
