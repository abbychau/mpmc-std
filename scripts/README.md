# Documentation Scripts

This directory contains scripts for generating and managing documentation for the MPMC Queue project.

## markdown-converter.js

A robust Node.js-based markdown to HTML converter that replaces the previous Python implementation.

### Features

- **Complete Markdown Support**: Headers (H1-H6), paragraphs, lists, code blocks, inline code
- **Inline Formatting**: Bold, italic, strikethrough, links, images
- **Code Syntax**: Fenced code blocks with language specification
- **List Handling**: Both ordered and unordered lists with proper nesting
- **HTML Safety**: Proper HTML escaping in content and code blocks
- **Performance**: Fast conversion without external dependencies

### Usage

```bash
# Convert markdown to HTML (output to stdout)
node scripts/markdown-converter.js input.md

# Convert markdown to HTML file
node scripts/markdown-converter.js input.md output.html
```

### Examples

```bash
# Convert a documentation file
node scripts/markdown-converter.js docs/ALGORITHM_DIAGRAMS.md docs/ALGORITHM_DIAGRAMS.html

# Preview conversion output
node scripts/markdown-converter.js README.md | head -20
```

## Requirements

- **Node.js**: Version 14.0.0 or higher
- **No external dependencies**: Uses only Node.js built-in modules

## Migration from Python

The previous implementation used a Python script embedded in the shell script. The new JavaScript implementation provides:

1. **Better Performance**: Faster startup and execution
2. **More Features**: Enhanced markdown parsing with better edge case handling
3. **Maintainability**: Cleaner, more readable code structure
4. **Extensibility**: Easier to add new markdown features or output formats

## Integration

The markdown converter is automatically used by `copy_benchmarks.sh` to convert documentation files from Markdown to HTML with proper styling and navigation.
