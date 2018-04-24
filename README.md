# Mediafire folder dumper
## Description
This tool dumps the folder structure in a mediafire link using the [Mediafire Core API](https://www.mediafire.com/developers/core_api/1.5/getting_started/)

## Usage
```bash
$ ./mediafire_dumper <folder_key>
```
where \<folder_key> can be a:
* Full URL with a folder fragment containing the folder_key like so `https://mediafire.com/Example#s8gh32jkjsh` 
* Just the folder_key itself `s8gh32jkjsh`

## Building
To build mediafire_dumper you'll need:
* Rust Nightly
* Cargo
```bash
$ cargo build
```
## TODO List
TODO: TODO List