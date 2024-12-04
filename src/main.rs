mod args;
use args::GenomeArgs;
use std::error::Error;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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

  #[derive(Debug, Clone, PartialEq, PartialOrd)]
  struct Limit {
    line: String
  }

  let fileopen = File::open(pathsam).expect("file not present");
  let fileread = BufReader::new(fileopen);
  let mut limit_lines = Vec::new();
  for i in fileread.lines(){
    let line = i.expect("line not found");
    if ! line.starts_with("@") {
      let iden = line;
      limit_lines.push(iden);
    }
  }
  let mut limit:Vec<Limit> = Vec::new();
  for i in limit_lines.iter(){
    let mutable = i.split(" ").filter(|x| !x.is_empty()).collect::<Vec<_>>();
    if mutable.is_empty() {
      continue
    } else {
      limit.push(Limit { line: mutable[2].to_string()});
    }
  }
  let mut sorted_id = File::create("samids.txt").expect("file not present");
  for i in limit.iter_mut(){
    write!(sorted_id, "{}\n", i.line).expect("not able to write th line");
  }
    Ok("The files have been written and the summary is given below".to_string())
    }
