# Recursively Find and Remove 👓🔥

This is a cross-platform utility that will recursively search a directory for files & folders with a specific name or names, find all occurrences then prompt you to delete them.

While you can achieve this in `bash` with a combination of `find` and `rm -rf`, the motivation for this tool is to use the same syntax for this operation across all operating systems and shell environments.

## Example

```
rrm -f "node_modules" -f ".DS_Store" ~/Development
```

Outputs:
```
Looking within:
  /home/alshdavid/Development

Looking for:
  node_modules
  .DS_Store

Found:
  /home/alshdavid/Development/my-project/another_one/.DS_Store
  /home/alshdavid/Development/my-project/node_modules
  /home/alshdavid/Development/my-project/.DS_Store

Delete matches? [y/N] y
  /home/alshdavid/Development/my-project/another_one/.DS_Store
  /home/alshdavid/Development/my-project/.DS_Store
  /home/alshdavid/Development/my-project/node_modules
```

# Development 🧩

## Prerequisite Tools

- [Rust](https://rustup.rs/)
- [Just](https://github.com/casey/just)

## Building

```
just build
target=release just build
```

## Running Development Build

```
just run
just run -f node_modules ~/Development
```