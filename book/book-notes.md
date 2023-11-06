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
