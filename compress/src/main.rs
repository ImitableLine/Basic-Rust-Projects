// Imports struct to use gzip algorithm to encode data (write)
use flate2::write::GzEncoder;
// Imports compression struct, letting you control the compression, (performance over speed decisions (by you))
use flate2::Compression;
// Imports file struct, which handles interaction with files. i.e. opening, reading, and writing.
use std::fs::File;
// Imports input/outpu (I/O) like Result. Read and Write are traits the define methods for reading and writing data.
use std::io::{self, Read, Write};

// Main is the basic entry point for rust, it is automatically run when program executes.
fn main() {
    // These two are both immutable and are string slices.
    // Just references to the data, the ownership remains with the binary itself, and are statically allocated in the program.
    let input_path = "compressme.txt"; // setting the directory, pointing it towards the text file I made.
    let output_path = "compressme.txt.gz"; // sets the output directory, where the compressed file will be saved.

    // This is the function call to the compress file function. It returns a "Result<()>"
    // the "if let Err(e)" is rusts error handling pattern. it check if the result is an error "(Err)". If there is an error, it binds the error to "e".
    if let Err(e) = compress_file(input_path, output_path) {
        // This is a macro which prints error messages to standard error "(stderr)" which is helpful for logging the failures.
        // it will ideally print the message below + error msg
        eprintln!("Error compressing file {}:", e)
    }
}

// declare function defining "compress_file" as a funcation.
// both parameters are &str, meaning it expects to references to slice's, ownership of the slice it not taken due to it being a reference.
// it takes two parameters. both of them a reference to a string slice, not taking ownership of either of them.
// The return type is io::Result<()>, which is a common idiom for error handling, "Result" is the return type.
// Result is a enum "Result<T, E>" can be "Ok(T)" which is success, or it can be "Err(E)" which is an error.
// the () respresents the unit type, which means function returns nothing upon success, just an acknowledgement that the operation completed.
fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    // open input_file for reading
    // the "?" is rust's error propagation, if there is an error the function will return the error, else it returns "Ok" than it assigns to input_file as normal.
    // File::open either returns an "Ok(File)" or an "Err(io::Error)".
    // The file instance is now owned by input_file, which is mutable. The function can read from it later.
    let mut input_file = File::open(input_path)?;

    // create the output_file where compressed data will go
    // File::create creates a new file, or truncates an existing one.
    // It returns a "Result<File>" and has a "?" meaning either "Ok" or "Err". Ok creats/truncates the file, which Err returns the error.
    // Ownership now dictates that output_file now has ownership over the new file.
    let output_file = File::create(output_path)?;

    // create a gzencoder to compress the input_file data
    // "Compressiong::default" dictates the default compression level ( speed or size/performance)
    // The ownership of "output_file" is now under "encoder". It is now no longer accessible as a "File", encoder has control over writing to it.
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    // read the input_file content
    // Creates a mutable and empty vector named "buffer" the type is Vec<u8>, dynamically allocated to hold the file's content.
    let mut buffer = Vec::new();
    // Read all the data from "input_file" into the buffer, the "?" handles errors like from above, (Ok(expectedresult) or Err(errortype))
    // buffer now takes ownership of reading the data
    input_file.read_to_end(&mut buffer)?;

    // write the new (compressed) data to the output_file
    // writes all the data from "buffer" into "encoder". encoder is set up to compress as it goes, the data is getting compressed as it writes to the "output_file".
    // this is just passing a reference to "buffer", so the ownership of "buffer" is not moved to encoder.
    encoder.write_all(&buffer)?;

    // finishes compression proccess. returning the underlying writer(the original output_file) ensuring that all data is properly flushed to disk.
    // not that compression is complete, encoder returns ownership of output_file, however this is not neccessary to the program, and it just circumstance.
    encoder.finish();

    // This just returns a message to state the operation as been successful, if an error occured before this, this will not show up.
    println!("File is compressed successfully!");

    // The function is setup to return io::Result<()>
    Ok(())
}
