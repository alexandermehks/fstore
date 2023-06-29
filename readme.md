## fstore 

fstore is a in filesystem key value store enabling multiple keys for same object

## Introduction
fstore is a command line infterface tool to add, edit, get and modify secrets all saved in the filesystem. 

Try the [online playground](https://ast-grep.github.io/playground.html) for a taste!

## Command line usage example

fstore has following form.
```
fstore --action [args]
```

### Example

* create a new object

```bash
fstore new objectname
```

* add a new secret to object
```bash
fstore set objectname password=mypassword
```

* get secret from object
```bash
fstore get objectname password
```

## CLI Screenshot

### Search
| Feature | Command | Screenshot |
| ------- | ------- | ---------- |
| Help  | `fstore` | ![image](https://github.com/alexandermehks/fstore/blob/main/assets/fstore_base.png) |
| Get  | `fstore get secretmangos` | ![image](https://github.com/alexandermehks/fstore/blob/main/assets/fstore_get.png) |
