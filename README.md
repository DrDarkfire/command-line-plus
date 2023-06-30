# command-line-plus
A multipurpose project that can execute solo or many file system tasks.

### Usage:
Currently, In your terminal you can run it and it will start in your home directory.
```
command-line-plus
```
You can specify a directory on whichever drive you want to start it in
```
command-line-plus E:\Path\To\Directory
```
You can run solo functions(this needs improved upon)
```
command-line-plus touch example1.txt example2.json example3.ts
```

### Funtioning commands:
#### cd
Change directory with the next directory or drive you want to work in. ".." will move you up a directory.
```
cd ..
cd nextpath
cd deeper\path
cd E:
```
#### touch
Specify one or multiple blank files you want to create in the current working directory
```
touch example.rs
touch example1.txt example2.json example3.ts
```

#### edit
Opens your default text editor with the specified file
```
edit example1.txt
```