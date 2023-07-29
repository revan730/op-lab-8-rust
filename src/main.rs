use std::{io::{self, Write, BufReader, BufRead}, fs::{File, OpenOptions}, cmp::Ordering, error::Error};

fn yes_no_prompt(prompt: &str) -> bool {
    let stdio = io::stdin();

    loop {
        println!("{}", prompt);
        let mut input = String::new();

        match stdio.read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                println!("Failed to read input, try again: {}", e);
                continue;
            },
        }

        let c = match input.chars().nth(0) {
            Some(cc) => cc,
            None => continue,
        };

        match c {
            'y' | 'Y' => return true,
            'n' | 'N' => return false,
            _ => continue,
        }
    }
}

fn open_files() -> Result<(File, File), io::Error> {
    let mut append = yes_no_prompt("Append (Y) or create new INPUT file (n)?:");
    let mut options = OpenOptions::new();
    let input_file: File = match options.read(true).write(true).create(!append).open("in") {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    append = yes_no_prompt("Append (Y) or create new OUTPUT file (n)?:");
    let mut options = OpenOptions::new();
    let output_file: File = match options.write(true).create(!append).open("out") {
        Ok(file) => file,
        Err(err) => return Err(err),
    };


    Ok((input_file, output_file))
}

fn write_to_input_file(file: &mut File) -> Result<(), io::Error> {
    println!("Write data to input file, press Ctrl-D to finish:");

    let stdio = io::stdin();
    loop {
        let mut input = String::new();
        match stdio.read_line(&mut input) {
            Ok(c) => {
                if c == 0 {
                    return Ok(());
                }
            },
            Err(err) => return Err(err),
        };

        let res = file.write_fmt(format_args!("{}", input));
        match res {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
    }
}

fn compare_words(a: &&str, b: &&str) -> Ordering {
    let length_cmp = b.len().cmp(&a.len());

    if length_cmp == Ordering::Equal {
        return b.cmp(&a);
    }

    length_cmp
}

fn find_and_write_longest(input_file: &mut File, output_file: &mut File) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(input_file);
    let words_counts = buf_reader.lines().map(|l| l.unwrap()).map(|line| {
        let mut words: Vec<_> = line.split(" ").collect::<Vec<&str>>();
        words.sort_by(compare_words);
        let longest_word = words[0];
        println!("Longest word: {}", longest_word);

        output_file.write_fmt(format_args!("{}\n", longest_word));

        words.len()
    }).reduce(|acc, e| acc + e);
    
    if words_counts.is_none() {
        return Err("File is empty".into());
    }

    match output_file.write_fmt(format_args!("{}\n", words_counts.unwrap())) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

fn main() {
    let (mut input_file, mut output_file) = open_files().expect("Failed to open files");
    write_to_input_file(&mut input_file).expect("Failed to write to input file");
    find_and_write_longest(&mut input_file, &mut output_file).expect("Failed to find longest words");
}
