# Recursively Find and Remove

This utility will search a directory for files / folders using a name specifier and delete them.

```
rrm -f node_modules ./my-project
rrm -f node_modules -f ./.DS_Store ./my-project
```

## Example

```
cd /home/alshdavid/Development
rrm -f node_modules -f ./.DS_Store ./my-project
```

Outputs:
```
Looking within:
  /home/alshdavid/Development/my-project

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
