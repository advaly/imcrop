## Summary

Image manipulation tool.

## Build
- cargo 1.60.0-nightly (95bb3c92b 2022-01-18)
- rustc 1.60.0-nightly (8cdb3cd94 2022-01-25)

## Usage

```
imcrop 0.1.1
ADVALY SYSTEM Inc.
Image manipulation tool

USAGE:
    imcrop [OPTIONS] <src file> <out file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --canvas <canvas>           Overlay input image on canvas with geometory 'WxH'
    -c, --crop <crop>               Cropping geometory 'WxH+X+Y'
    -o, --overlay <overlay file>    Overlay a transparent image on the final image
    -s, --resize <resize>           Resizing geometory 'WxH'
    -r, --rotate <rotate>           Rotate 90/180/270

ARGS:
    <src file>    Input image file
    <out file>    Output image file
```

## Note

Processing order is as follows.

1. Rotate
1. Canvas
1. Crop
1. Resize
1. Overlay
