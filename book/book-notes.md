# Chapter 2

```
seq 5 | parallel seq {} '>' example.{}
```
Generates a list of numbers then creates a file for each number. Inside the file is a list of positive integers leading up to that number.

```
parallel echo ::: 1 2 3 4 5
```
Prints the input numbers in parallel.

```
parallel echo counting lines ';' wc -l ::: example.*
```
We can use sequential commands with parallel. We need to enclose special characters with quotations. The shell will automatically use glob expansion when we use *. So therefore this command will find each example file then print our string followed by the linecount.

```
parallel echo counting {} ';' wc -l {} ::: example.*
```
We can use each value passed into parallel using {}.

```
parallel echo counting {1} {2} ';' wc {1} {2} ::: -l -c ::: example.*
```
When you have multiple inputs parallel will use each combination of inputs.
You can refer to the different sets of inputs using numbers.

```
parallel --dry-run echo counting {1} {2} ';' wc {1} {2} ::: -l -c ::: example.*
```
Dry run will print out all of the commands that will be run.

```
parallel -k sleep {} ';' echo {} done ::: 5 4 3 2 1
```
Paralle doesn't guarantee the output will be in the same order as the input. If you need this behaviour you can use the flag -k or --keep-order. This won't slow down your program. The commands will still be run in parallel but later values will wait to be printed until all earlier ones are done.

