mod args;
use args::GenomeArgs;
use clap::Parser;
use rayon::prelude::*;
use std::error::Error;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-12-4

rust-samtools: allows for the generation of the ids such samtools fastid.
This will generate all over the fastid instead of the specific range.

* */

fn main() {
    let args = GenomeArgs::parse();
    let samtools_start = samtools_fastid(&args.alignment_arg).unwrap();
    println!("The selected region has been written: {}", samtools_start);
}
fn samtools_fastid(pathsam: &str) -> Result<String, Box<dyn Error>> {
    let fileread: Vec<String> = read_to_string(pathsam)
        .expect("failed to read")
        .lines()
        .map(String::from)
        .collect();
    let limit_lines = fileread
        .par_iter()
        .map(|x| processline(x))
        .collect::<Vec<_>>();

    let mut limit: Vec<_> = limit_lines
        .par_iter()
        .map(|x| x.join(""))
        .map(|x| linesplit(x.as_str()))
        .filter(|x| !x.is_empty())
        .map(|x| x[2].to_string())
        .collect::<Vec<_>>();

    let mut sorted_id = File::create("samids.txt").expect("file not present");
    for i in limit.iter_mut() {
        writeln!(sorted_id, "{}", i).expect("not able to write th line");
    }
    Ok("The files have been written and the summary is given below".to_string())
}

fn processline(pathstr: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    if !pathstr.starts_with("@") {
        output.push(pathstr.to_string());
    }
    output
}

fn linesplit(pathstr: &str) -> Vec<String> {
    let output = pathstr
        .split("\t")
        .filter(|x| !x.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();
    output
}
