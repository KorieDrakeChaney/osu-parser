# osu!parser

This is a parser for osu!. It allows you to read and manipulate osu files in a simple efficient way.

## Features

- Read and write osu files.
- Read and write osb files.

## Usage

### Beatmap(osu file)

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

### Storyboard(osb file)

```rs
use osu_parser::Storyboard;

fn main() {
    let storyboard = Storyboard::from_file("path/to/storyboard.osb").unwrap();
    // Save the storyboard
    storyboard.save();
    // Prints the storyboard
    println!("{}", storyboard);
}
```

## Upcoming Features

- Add `.osk` file support
- Add `.osr` file support

`.osz` file support is supported in my other crate [osu_unzip](https://github.com/KorieDrakeChaney/osu-unzip)
