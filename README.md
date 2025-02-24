## Summary

Image manipulation tool.

## Build
- cargo 1.83.0 (5ffbef321 2024-10-29)
- rustc 1.83.0 (90b35a623 2024-11-26)

## Usage

```
imcrop 0.1.4
ADVALY SYSTEM Inc.
Image manipulation tool

USAGE:
    imcrop [OPTIONS] <src file> <out file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --brightness <brightness>    Adjust brightness by i32. Negative values decrease and positive values increase
    -b, --canvas <canvas>            Overlay input image on canvas with geometory 'WxH'
        --combine <combine>          Combine multiple images into one image. Each image size must be same. 'h' or 'v'
        --contrast <contrast>        Adjust contrast by f32. Negative values decrease and positive values increase
    -c, --crop <crop>                Cropping geometory 'WxH+X+Y'
    -o, --overlay <overlay file>     Overlay a transparent image on the final image
    -q, --quality <quality>          Output image quality in the range 1-100 where 1 is the worst and 100 is the best
    -s, --resize <resize>            Resizing geometory 'WxH'
    -r, --rotate <rotate>            Rotate 90/180/270

ARGS:
    <src file>    Input image file. Separate by commas to specify multiple files in combine mode
    <out file>    Output image file
```

## Note

Processing order is as follows.

1. Combine
1. Rotate
1. Crop
1. Resize
1. Brightness
1. Contrast
1. Canvas
1. Overlay
