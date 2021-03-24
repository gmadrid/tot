Read of file of the form:

```
name:value
name2:value2
name3:value4

name:other
name2:other2
name3:other4
```

and output this:

```
name\tname2\tname3\n
value\tvalue2\tvalue4\n
other\tother2\tother4\n
```

* a blank line indicates a new record
* two blank lines does _not_ output an empty record
