# loghawk
A cli tool to display large CSV files

![Screenshot of LogHawk](https://github.com/janstarke/loghawk/blob/main/docs/sample.png?raw=true)

## Features 

### Scrolling
The tool is expected to be used by forensics analysts. The idea is that you have a file, csv or txt,
where you have a *first column* which is important for you, such as as timestamp. The remaining lines
might be long, so you want to scroll to the right without losing the first column.

## Usage

```
Usage: loghawk [OPTIONS] [FILE] [DELIMITER]

Arguments:
  [FILE]       file to display, use '-' for stdin [default: -]
  [DELIMITER]  delimiter for CSV and TXT formats [default: ,]

Options:
  -F, --format <FILE_FORMAT>  format of the input file [default: csv] [possible values: csv, txt]
  -h, --help                  Print help
  -V, --version               Print version
```