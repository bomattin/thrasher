# Example Template

This page gives an example of the template file formats used by thrasher to
generate binary files. Templates consist of 2 files, the binary template itself
(.bint file) and the corresponding Python script (.pyt). Here we use the PNG
file format as the example.

## png.bint
```
# Magic number
89 50 4e 47 0d 0a 1a 0a
# IHDR chunk, 128x128 truecolor, no filt, no comp, no interleave
00 00 00 0D 49 48 44 52 00 00 00 80 00 00 00 80 08 02 00 00 00 {crc_ihdr:x:4}
# IDAT chunk
{len_data:x:4} 49 44 41 54 {data:x:len_data} {crc_data:x:4}
# IEND chunk, hardcoded CRC32
00 00 00 00 49 45 4E 44 AE 42 60 82
```

The format strings used are Rust format strings. Variables referenced in the
binary template MUST be defined in the corresponding .pyt file.

## png.pyt
