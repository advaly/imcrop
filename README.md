## Summary

Image cropping tool.


## Usage

```
$ imcrop geometry src_file out_file
```

Argument | Description
-- | --
geometry | Cropping region with format of "`Width`x`Height`+`x`+`y`"
src_file | Input image file
out_file | Cropped output file

The format of `geometry` is same as ImageMagick.

e.g. "200x100+15+30" represents to crop 200 width x 100 Height region with offset to upper left point of (15, 30).

You can save output image file as different image type. For example, arguments with src_file = "hoge.jpg" and out_file = "fuga.png" means input JPEG image format and output PNG image format.
