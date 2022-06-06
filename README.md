# Interpreter
of my own language in Rust.

Described in `about.md` (polish, with examples)
and in `documentation.md` (english, broader, but no examples)

Grammar in `grammar.md`

# Tasks
- [x] Prepare repository and workflow
- [x] Formalize language
- [x] Implement lexer
- [x] Implement parser
- [x] Implement interpreter
- [ ] ~~Party~~ nah, need to pass finals :')

# How to use:
1. [Download](https://github.com/MiniaczQ/interpreter/releases) distribution for your OS (Windows or Linux)
2. Run the interpreter through command line

# Usage example
## Linux Bash
`./interpreter -i < source.txt`
`./interpterer -f source.txt`

## Windows Powershell
`Get-Content -Path source.txt | .\interpreter.exe -i`
`.\interpreter.exe -f source.txt`
