# osu!parser

This is a parser for osu!. It allows you to read and manipulate osu files in a simple efficient way.

## Features

- Read and write osu files.

## Usage

```rs
use osu_parser::Beatmap;

fn main() {
    let beatmap = Beatmap::from_file("path/to/beatmap.osu").unwrap();
    // Change the title of the beatmap
    beatmap.change_metadata_title("@KorieDrakeChaney was here");
    // Save the beatmap to the directory it was originally loaded from
    beatmap.save_to_directory();
    // Prints the beatmap
    println!("{}", beatmap);
}
```

## Upcoming Features

- Add `.osk` file support
- Add `.osb` file support
- Add `.osr` file support

`.osz` file support is supported in my other crate [osu_unzip](https://github.com/KorieDrakeChaney/osu-unzip)
