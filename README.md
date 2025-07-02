# CLI Dictionary (CLI-словарь)

A simple command-line dictionary application that allows you to store and retrieve word translations. The dictionary uses a local database to persist the translations.

## Features

- Add new words with their translations
- Look up translations for specific words
- List all words in the dictionary
- Search words by prefix
- Persistent storage using sled database

## Installation

Make sure you have Rust installed on your system. Then clone the repository and build the project:

```bash
cargo build --release
```
The compiled binary will be available in `target/release/dictionary`
## Usage
``` bash
dictionary <COMMAND>
```
### Available Commands
#### Add a new word
``` bash
dictionary add <WORD> <TRANSLATION>
```
Example:
``` bash
dictionary add hello привет
```
#### Get translation
``` bash
dictionary get <WORD>
```
Example:
``` bash
dictionary get hello
```
#### List all words
``` bash
dictionary list
```
#### Search words by prefix
``` bash
dictionary search <PREFIX>
```
Example:
``` bash
dictionary search he
```
### Options
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Database
The dictionary uses sled database for storage. The database files are stored in the `my_db` directory in your current working directory.
## Examples
``` bash
# Add a new word
dictionary add cat кошка

# Look up a translation
dictionary get cat

# Add another word
dictionary add dog собака

# List all words
dictionary list

# Search words starting with 'c'
dictionary search c
```
## Error Handling
The application will display appropriate error messages in cases such as:
- Word not found in the dictionary
- Database access errors
- Invalid UTF-8 encoding in stored data

## Dependencies
- clap (v4.5.40): Command line argument parser
- sled (v0.34.7): Embedded database
