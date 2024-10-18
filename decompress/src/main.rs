// use imports items like structs functions and modules
// the gzencoder specifies the full path of GzDecoder struct from flate2
// GzDecoder is used to decompress files that are compressed with the gz file format
use flate2::read::GzDecoder;

// brings the "File" struct from the "std"/standard library into scope.
// File is used to open and handle file operations like read and write.
use std::fs::File;

// std::io includes basic input/output from std
// "self:" allows the use of "io::Result" for error pattern matching
// Read is a trait, and allows reading data from a source/file or buffer, needed to read the data stream.
use std::io::{self, Read, Write};

// fn main is the entry point automatically run when main.rs is executed
// -> specifies return type
// io::Result is a type alias for "Result<(), io::Error>" it is an enum for error pattern matching.
// Result can have Ok() or Err(E). OK in this instance just returns an affirmation of success, but it can return whatever the fn would.
// Err(E) will return the error in readable format.
// () is effectivly void from other languages.
fn main() -> io::Result<()> {
    // let is creating a new variable contained the read file taken from path parameter.
    // File::open is a method called to open a file for reading.
    // "File" comes from std::fs and is a standard module for file system operations
    // the file path is a string slice
    // is succeed, it returns the result enum (Ok(File), Err(R))
    // A possible error here is file not found.
    // The "?" is error propagation, if there is an error it will return that error immediatly.
    let compressed_file = File::open("./compressme.txt.gz")?;

    // new mutable variable called decoder
    // mut is needed as reading data from the decoder requires mutability
    // GzDecoder is a struct from flate2 crate, it is for compression with Gz
    // GzDecoder::new() takes an input source that implements the read trait, in this instance a file object
    // compressed_file is passed to GzDecoder::new() reading the compressed data and than decompressing it
    // "decoder" var is now ready to decompress the data in "compressed_file"
    let mut decoder = GzDecoder::new(compressed_file);

    // let mut created a new mutable var, in this case "output_file" which holds the new file that will have the de-compressed data
    // if the file already exists, it will be overwritten
    // File::create() returns the io::Result enum like usual
    // the "?" is error propagation which returns the error to the caller.
    // when this runs with no errors, it will create a "decompressed_file.txt" that will store the data
    let mut output_file = File::create("decompressed_file.txt")?;

    // let mut creates a mutable var, in this case a vector called "buffer"
    // "buffer" needs to be mutable, as it will read and then write the data
    // Vec is <u8> in this instance, and is a growable array of bytes
    // the buffer will store chunks of the decompressed data before writing it to the output file
    let mut buffer = Vec::new();

    // read_to_end will read all the data in decoder, again storing chunks in a mutable reference to buffer
    // read_to_end has error pattern matching as usual, and this the "?" propagation will return the error if one is found
    decoder.read_to_end(&mut buffer)?;

    // write all of the information in a reference to buffer (chunks of decompressed data) to output_file
    // buffer is a reference, and does not need to write as this point, as such it is immutable
    // write_all results error pattern matching
    output_file.write_all(&buffer)?;

    // println, the "!" is a macro and is evaluated at compile time.
    println!("Decompression complete! Output written to 'decompressed_file.txt'");

    // this "main" function is expecting a result as its return type, as eariler pattern matching
    // simply returns Ok() as operation has succeded.
    Ok(())
}
