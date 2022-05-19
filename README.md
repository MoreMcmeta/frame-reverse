# frame-reverse
**frame-reverse** is a simple command line tool written in Rust. It converts MoreMcmeta animation strips from this format:
```
0 4
1 5
2 6
3 7
```

to this format:
```
0 1
2 3
4 5
6 7
```

## Options
Run the tool with the `--help` flag to view a list of the tool's options.

* `i` - path to image to use as input
* `o` - path where new file will be written. Any existing file at this location will be overwritten.
* `w` - width of a **frame** in the image. This is not the width of the entire image.
* `h` - height of a **frame** in the image. This is not the height of the entire image.
* `r` - number of frames per row in the output image. Optional. If not provided, the output image will have the same number of frames per row as the input image.

For example,
```shell
$ ./frame-reverse -i myimage.png -o output.png -w 10 -h 10 -r 2
```
will reverse the ordering of the frames of the image `myimage.png`, where each frame is 10x10, to create `output.png` with two 10x10 frames per row.

## Building the Tool
[Install Rust](https://www.rust-lang.org/tools/install). Then run `cargo build` or `cargo build --release` (for an optimized build) from the command line.