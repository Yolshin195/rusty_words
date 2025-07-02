``` markdown
# Rusty Words

A simple command-line dictionary application that allows you to store and retrieve word translations. The dictionary uses a local database to persist the translations.

## Features

- Add new words with their translations
- Look up translations for specific words
- List all words in the dictionary
- Search words by prefix
- Persistent storage using sled database

## Installation

Make sure you have Rust installed on your system. Then clone the repository and build the project:
```
bash cargo build --release
``` 
The compiled binary will be available in `target/release/rusty_words`

## Usage
```
bash rusty_words
command
``` 

### Available Commands

#### Add a new word
```
bash rusty_words add
word
translation
translation
word
``` 
Example:
```
bash rusty_words add hello привет
``` 

#### Get translation
```
bash rusty_words get
word
word
``` 
Example:
```
bash rusty_words get hello
``` 

#### List all words
```
bash rusty_words list
``` 

#### Search words by prefix
```
bash rusty_words search
prefix
prefix
``` 
Example:
```
bash rusty_words search he
``` 

### Options
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Database
The dictionary uses sled database for storage. The database files are stored in the `my_db` directory in your current working directory.

## Examples
```
bash
# Add a new word
rusty_words add cat кошка
# Look up a translation
rusty_words get cat
# Add another word
rusty_words add dog собака
# List all words
rusty_words list
# Search words starting with 'c'
rusty_words search c
``` 

## Error Handling
The application will display appropriate error messages in cases such as:
- Word not found in the dictionary
- Database access errors
- Invalid UTF-8 encoding in stored data

## Dependencies
- clap (v4.5.40): Command line argument parser
- sled (v0.34.7): Embedded database
```
