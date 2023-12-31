## fstore 

fstore is a in filesystem key value store enabling multiple keys for same object

## Introduction
fstore is a command line infterface tool to add, edit, get and modify secrets all saved in the filesystem. 

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

* export all objects as csv
```bash
fstore export
```

## CLI Screenshot

### Search
| Feature | Command | Screenshot |
| ------- | ------- | ---------- |
| Help  | `fstore` | ![image](https://github.com/alexandermehks/fstore/blob/main/assets/fstore_base.png) |
| Get  | `fstore get secretmangos` | ![image](https://github.com/alexandermehks/fstore/blob/main/assets/fstore_get.png) |
| Get  | `fstore get secretmangos ultimatemango` | ![image](https://github.com/alexandermehks/fstore/blob/main/assets/fstore_get_specific.png) |
