extern crate clap;
extern crate libflate;
extern crate filetime;

use clap::{App, Arg};
use libflate::gzip;
use filetime::FileTime;

fn read() -> String {
    //*
    // Simple read function for obtaining user input
    // Returns user input as String, stripped from newline chars.
    //*
    
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    // Remove newline from input, safer to use trim than pop...
    let user_input = user_input
        .trim_right_matches(|c| c == '\n' || c == '\r');
    return String::from(user_input);
}

fn gzip(uncompressed_file: &str) -> String {
    //*
    // Try to create gzip archive from uncompressed_file
    // See:
    // https://github.com/sile/libflate/blob/master/examples/flate.rs
    // Return values:
    // The name of the compressed file as String
    //*
    
    // Input file
    let input_filename = uncompressed_file;
    
    // Get metadata
    let metadata = std::fs::metadata(input_filename).unwrap();
    // Save original file's timestamps here
    let mtime = FileTime::from_last_modification_time(&metadata);
    let atime = FileTime::from_last_access_time(&metadata);
    // Create Path
    let gzip_path = std::path::Path::new(input_filename);
    println!("{:?}", gzip_path);
    
    
    let input: Box<std::io::Read> = Box::new(std::fs::File::open(input_filename)
                                        .expect(&format!("Can't open file: {}", input_filename)));

    let mut input = std::io::BufReader::new(input);

    // Output file
    // Create String type from argument
    let mut tmp_output_filename = String::from(uncompressed_file);
    // Append '.gz' to String
    tmp_output_filename.push_str(".gz");
    // Typecast the proper output name back into &str type
    let output_filename = &tmp_output_filename;
    let output: Box<std::io::Write> = Box::new(std::fs::File::create(output_filename)
                                          .expect(&format!("Can't create file: {}", output_filename)));
    let output = std::io::BufWriter::new(output);

    let mut encoder = gzip::Encoder::new(output).unwrap();
    std::io::copy(&mut input, &mut encoder).expect("Encoding GZIP stream failed");
    encoder.finish().into_result().unwrap();
    
    return output_filename.to_owned();
}

fn validate_age(age: &str) -> Result<u64, String> {
    //*
    // Validate user input. Should be positive int followed by 'd|h|m'
    // Ok examples: 2d, 30m, 1h
    // Return values:
    // Ok: Time specified as seconds, ex: "2m" returns 120
    // Err: Invalid time, or time format.
    //*
    
    let mut age_string = String::from(age);
    // Time format (d,h,m) saved below
    let age_string_timeformat = age_string.pop();
    // Validate that we have a positive int left after pop
    if age_string.parse::<u64>().is_err() {
        return Err(format!("Error: {} is not an integer.", age_string)
            .to_string());
    }
    
    // Correct int assigned below
    let age_int = age_string.parse::<u64>().unwrap();

    // Validate time format
    match age_string_timeformat {
        Some('d') => return Ok(86400*age_int), // Days in seconds
        Some('h') => return Ok(3600*age_int), // Hours in seconds
        Some('m') => return Ok(60*age_int), // Minutes in seconds
        _ => return Err(format!("Error. Invalid time format: {}. Use: h,m,s.", 
                age_string_timeformat.unwrap()).to_string()),
    };
}

fn main() {
    // Pure Rust implentation of something like:
    // find <PATH> -maxdepth 1 -type f -name "*.log" -mtime +100 -exec gzip "{}" \+
    static INFO: &'static str = "
    sagz: search and gzip
    
    EXAMPLES:
    sagz -p ./ -a 10m -e .log
    Locate and gzip files in current directory older than 10 minutes,
    with extension .log
    sagz -p /home/magnus/slask/ -a 100d -d
    Locate all files (no -e switch) in /home/magnus/slask/, older than 100 days,
    do not gzip (-d flag), just print which files would be processed.
    
    NOTES:
    When running without extension, you will be prompted as an extra
    precaution, since this means you are processing all files in the
    path.
    
    Unlike GNU gzip, timestamps are not preserved upon creating gzip.";
    
    // Configure arguments
    let matches = App::new("sagz")
                      .version("1.0")
                      .author("Magnus Wallin <magnuswallin@tutanota.com>")
                      .about(INFO)
                      .arg(Arg::with_name("age")
                           .short("a")
                           .long("age")
                           .value_name("TIME")
                           .help("Operate on files older than <TIME>.\n\
                            Valid <TIME> is positive integer,\n\
                            followed by d (days), or h (hours), or m (minutes).\n\
                            Examples: 2d, 13h, 45m")
                           .takes_value(true)
                           .required(true))
                      .arg(Arg::with_name("ext")
                           .short("e")
                           .long("ext")
                           .value_name("EXT")
                           .help("Operate on files with extension <EXT>.\n\
                            Examples: .log, .bak\n\
                            If not supplied, work on ALL files.")
                           .takes_value(true))
                      .arg(Arg::with_name("path")
                           .short("p")
                           .long("path")
                           .value_name("PATH")
                           .help("Operate on files in path <PATH>.\n\
                            Examples: ./, /home/backups")
                            .takes_value(true)
                            .required(true))
                      .arg(Arg::with_name("d")
                           .short("d")
                           .long("dryrun")
                           .help("Don't compress. Just print which files\n\
                           would be compressed, and their age in seconds"))
                      .arg(Arg::with_name("k")
                           .short("k")
                           .long("keep")
                           .help("Don't delete the original file."))
                      .get_matches();

    // Parse arguments
    let age = matches.value_of("age").unwrap();
    let extension = matches.value_of("ext").unwrap_or("");
    let temp_path = matches.value_of("path").unwrap();
    let dryrun = matches.occurrences_of("d");
    let keep = matches.occurrences_of("k");
    
    // Sanity check the age input from user
    let age_in_seconds = validate_age(&age).unwrap();

    // Check that we have a valid path
    if std::fs::metadata(&temp_path).is_err() {
        println!("Error: The path {} is not valid", temp_path);
        std::process::exit(1);
    }

    // Prompt if no extension is set
    if extension.is_empty() {
        loop {
            println!("No extension supplied. You will run on all files in {}", temp_path);
            println!("Type 'yes' to continue, 'no' to exit: ");
            let confirm = read();
            match confirm.to_lowercase().as_str() {
                "yes" => break,
                "no" => {
                            println!("Exiting.");
                            std::process::exit(0);
                        },
                _ => println!("Incorrect input: '{}'", confirm), 
            };
        }
    }
    
    // Use this path
    let path = std::fs::read_dir(&temp_path).unwrap();
    
    // Save number of files we work on here
    let mut num_files_processed = 0;

    // Loop directory
    for file in path {
        // Save file object here
        let file_obj = file.unwrap();
        // Save filename here
        let file_name = file_obj.path()
            .display()
            .to_string();
        // Get metadata
        let file_metadata = std::fs::metadata(file_obj.path()).unwrap();
        // Only work on files with given extension
        if file_metadata.is_file() && file_name.ends_with(&extension) {
            let mtime = file_metadata
                .modified().unwrap()
                .elapsed().unwrap()
                .as_secs();
            // Is file older than 'age_in_seconds'?
            if mtime > age_in_seconds {
                num_files_processed+=1;
                // Do we have dryrun flag?
                if dryrun == 1 {
                    println!("Matched file: {}", file_name);
                    continue;
                }
                gzip(&file_name);
                // Do not delete if -k flag is used
                if keep != 1 {
                    std::fs::remove_file(&file_name).unwrap();
                }
            }
        }
    }
    if num_files_processed < 1 {
        println!("No files found matching the arguments.");
    } else {
        println!("{} files processed.", num_files_processed);
    }
}

