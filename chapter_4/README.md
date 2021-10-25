# Ownership
The big thing with Rust is ownership and how memory is handled. In this programming language, a variable in the stack vs. in heap could operate differently. 

Data stored on the stack must be of a known and fixed size. Otherwise, that data would be stored on in heap memory. Heap memory is a lot less precise than the stack. 

## Rules of Ownership
- Each value has a variable which is it's owner
- There is only one owner
- If owner goes out of scope the value is dropped


Double free error: When two variables point to the same object and the variables leave scope, both variables will attempt to free the same space. Rust takes care of this by making the first variable "not valid" and does not worry about freeing up the memory. This is called a **move**.

Functions will free up memory of an object such as a String. Make sure to see if a function gives ownership back after the function call. This happens because when the String is passed as an argument it is treated as a **move**. Luckily, there are **references** which help us not worry about losing ownership when dealing with functoins!
