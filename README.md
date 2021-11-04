## Summary

Image manipulation tool.


## Usage

```
imcrop 0.1.0
ADVALY SYSTEM Inc.
Image manipulation tool

USAGE:
    imcrop [OPTIONS] <src file> <out file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --crop <crop>        Cropping geometory 'WxH+X+Y'
    -s, --resize <resize>    Resizing geometory 'WxH'
    -r, --rotate <rotate>    Rotate 90/180/270

ARGS:
    <src file>    Input image file
    <out file>    Output image file
```

## Note

Processing order is as follows.

1. Rotate
1. Crop
1. Resize
