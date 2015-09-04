use std::io::prelude::*;
use std::{io, fs, thread};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::sync::Arc;

#[derive(Clone,Copy)]
pub enum OutputMode {
    Print,
    SortAndPrint,
    Count,
}

use self::OutputMode::*;

pub struct Options {
    pub files: Vec<String>,
    pub pattern: String,
    pub output_mode: OutputMode,
}

fn read_files(options: Arc<Options>, out_channel: SyncSender<String>) {
    for file in &options.files{
        let file = fs::File::open(file).unwrap();
        let file = io::BufReader::new(file);
        for line in file.lines() {
            let line = line.unwrap();
            out_channel.send(line).unwrap();
        }
    }
}

fn filter_lines(options: Arc<Options>, 
                in_channel: Receiver<String>,
                out_channel: SyncSender<String>) {
    for line in in_channel.iter() {
        if line.contains(&options.pattern) {
            out_channel.send(line).unwrap();
        }
    }
}

fn output_lines(options: Arc<Options>, in_channel: Receiver<String>) {
    match options.output_mode {
        Print => {
            for line in in_channel.iter() {
                println!("{}", line);
            }
        },
        Count => {
            let count = in_channel.iter().count();
            println!("{} hits for {}.", count, options.pattern);
        },
        SortAndPrint => {
//            let mut data: Vec<String> = in_channel.iter().collect();
            unimplemented!()
        }
    }
}

pub fn run(options: Options) {
    let opt = Arc::new(options);

    let (line_sender, line_receiver) = sync_channel(16);
    let (filtered_sender, filtered_receiver) = sync_channel(16);

    let r_opt = opt.clone();
    let h1 = thread::spawn(move || read_files(r_opt, line_sender));
    let f_opt = opt.clone();
    let h2 = thread::spawn(move || filter_lines(f_opt, line_receiver, filtered_sender));
    let p_opt = opt.clone();
    let h3 = thread::spawn(move || output_lines(p_opt, filtered_receiver));

    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
}


fn main() {
    let options = Options {
        files: vec!["src/main.rs".to_string()],
        pattern: "let".to_string(),
        output_mode: Print };
        run(options);

}
