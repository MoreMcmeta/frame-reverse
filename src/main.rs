use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;
use clap::Parser;
use image::{GenericImage, GenericImageView, ImageBuffer};
use crate::PosIntError::{ParseError, ZeroError};

/// Represents errors converting a string to a positive (non-zero, non-negative) integer
#[derive(Debug)]
enum PosIntError {

    /// Error parsing a string to a non-negative integer
    ParseError(ParseIntError),

    /// The integer is zero.
    ZeroError

}

impl Error for PosIntError {}

impl fmt::Display for PosIntError {

    /// Formats this error as a string.
    ///
    /// # Arguments
    ///
    /// * `f` - formatter
    ///
    /// # Errors
    ///
    /// Returns a formatting error if this error could not be written to the formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError(err) => write!(f, "{}", err),
            ZeroError => write!(f, "Integer cannot be zero")
        }
    }

}

impl From<ParseIntError> for PosIntError {

    /// Converts a `ParseIntError` from the standard library to the more general `PosIntError`.
    ///
    /// # Arguments
    ///
    /// * `err` - error to convert
    fn from(err: ParseIntError) -> Self {
        ParseError(err)
    }

}

/// Parses a string as a positive (non-negative, non-zero) 32-bit integer.
///
/// # Arguments
///
/// * `str` - string to parse as a positive integer
///
/// # Errors
///
/// The function returns a `PosIntError::ParseError` if the integer cannot be
/// parsed into an unsigned 32-bit integer or a `PosIntError::ZeroError` if the
/// integer is zero.
fn parse_positive_int(str: &str) -> Result<u32, PosIntError> {
    let result = u32::from_str(str)?;

    match result == 0 {
        true => Err(ZeroError),
        false => Ok(result)
    }
}

/// Contains program arguments parsed from the command line
#[derive(Parser)]
#[clap(author, version, about)]
struct FrameRev {

    /// Path to input image
    #[clap(parse(from_os_str), short = 'i', long = "input")]
    input: std::path::PathBuf,

    /// Width of a frame in the image
    #[clap(parse(try_from_str = parse_positive_int), short = 'w', long = "width")]
    frame_width: u32,

    /// Height of a frame in the image
    #[clap(parse(try_from_str = parse_positive_int), short = 'h', long = "height")]
    frame_height: u32,

    /// Number of frames per row in the destination image
    #[clap(parse(try_from_str = parse_positive_int), short = 'r', long = "frames-per-row")]
    frames_per_row: Option<u32>,

    /// Path to location whose contents will be overwritten with output
    #[clap(parse(from_os_str), short = 'o', long = "output")]
    output: std::path::PathBuf,

}

/// Divides two unsigned integers, always rounding up if there is a fractional remainder.
///
/// # Arguments
///
/// * `dividend` - number to divide
/// * `divisor` - number to divide the dividend by
fn div_ceil(dividend: u32, divisor: u32) -> u32 {
    dividend / divisor + ((dividend % divisor != 0) as u32)
}

/// Runs the CLI.
///
/// # Panics
///
/// Panics if the provided command-line arguments are invalid (see [`FrameRev`]), if
/// the frame width or frame height are larger than the image dimensions, if the
/// input location cannot be read from, or if the output location cannot be written to.
fn main() {
    let args = FrameRev::parse();

    let source = image::open(args.input)
        .expect("Image not found");
    if args.frame_width > source.width() || args.frame_height > source.height() {
        panic!("Frames cannot be larger than source image");
    }

    // Determine source image dimensions
    let src_frames_x = source.width() / args.frame_width;
    let src_frames_y = source.height() / args.frame_height;

    // Determine destination image dimensions
    let total_frames = source.width() / args.frame_width * source.height() / args.frame_height;
    let dest_frames_x = match args.frames_per_row {
        Some(num) => num,
        None => source.width() / args.frame_width
    };
    let dest_frames_y = div_ceil(total_frames, dest_frames_x);

    let mut dest = ImageBuffer::new(
        dest_frames_x * args.frame_width,
        dest_frames_y * args.frame_height
    );

    for x in 0..src_frames_x {
        for y in 0..src_frames_y {

            // Determine pixel offsets of frames in source image
            let source_offset_x = x * args.frame_width;
            let source_offset_y = y * args.frame_height;

            // Determine pixel offsets of frames in destination image
            let frames_processed = x * src_frames_y + y;
            let dest_offset_x = (frames_processed % dest_frames_x) * args.frame_width;
            let dest_offset_y = (frames_processed / dest_frames_x) * args.frame_height;

            // Copy source frame to destination image
            let source_frame = source.view(
                source_offset_x,
                source_offset_y,
                args.frame_width,
                args.frame_height
            );
            dest.copy_from(
                source_frame.deref(),
                dest_offset_x,
                dest_offset_y
            ).expect("Unable to copy frame");

        }
    }

    dest.save(args.output).expect("Unable to save image");
}
