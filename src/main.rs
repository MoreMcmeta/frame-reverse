use std::ops::Deref;
use clap::Parser;
use image::{GenericImage, GenericImageView, ImageBuffer};

#[derive(Parser)]
struct FrameRev {
    #[clap(parse(from_os_str), short = 'i', long = "input")]
    input: std::path::PathBuf,

    #[clap(short = 'w', long = "width")]
    frame_width: u32,

    #[clap(short = 'h', long = "height")]
    frame_height: u32,

    #[clap(parse(from_os_str), short = 'o', long = "output")]
    output: std::path::PathBuf,
}

fn main() {
    let args = FrameRev::parse();
    let source = image::open(args.input)
        .expect("Image not found");

    let frames_x = source.width() / args.frame_width;
    let frames_y = source.height() / args.frame_height;

    let mut dest = ImageBuffer::new(
        frames_x * args.frame_width,
        frames_y * args.frame_height
    );

    for x in 0..frames_x {
        for y in 0..frames_y {
            let source_offset_x = x * args.frame_width;
            let source_offset_y = y * args.frame_height;

            let frames_processed = x * frames_y + y;
            let dest_offset_x = (frames_processed % frames_x) * args.frame_width;
            let dest_offset_y = (frames_processed / frames_x) * args.frame_height;

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
