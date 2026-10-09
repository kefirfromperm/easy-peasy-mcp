# Easy-Peasy MCP Server
The main goal is to allow to an agent to find a file or files with complex criteria.

I will use RUST

## Commands

- find
- get config
- update config

## Request
The request contains fields

- working directory
- search criteria
- output format

### Working directory
One or more paths to search in

### Search criteria

Search criteria is a query with multiply conditions which can be combined with AND, OR, XOR and NOT logical operators.

A condition can be applied on

- file path
- file name with extension
- file last change date
- file size
- file content
- string size
- string content (if two or more conditions are applied to the string content means that all fo these conditions must be applied to the same string in a file)

Condition operators

- exact match
- contains
- starts with
- ends with
- regex
- wildcard (for the file path and file name with extension)
- algebraic operators for sizes and dates

Escape operators can be applied for the string content in conditions

Exclude block to allow to support comments in programming languages.

### Output format

Output can contain fields

- exists YES/NO (true if at least one file is found)
- files count
- string count
- matches count

Per file fields

- string count
- matches count
- matched fragments with
- matched fragments line numbers
- the whole strings of matched fragments with strings before and after with size how many strings before and after have 
  to show

Also output format includes

- max file count
- max string count
- max matches count

## Configuration

The configuration contains default values for

- working directory (by default it's only current working directory)
- output format
