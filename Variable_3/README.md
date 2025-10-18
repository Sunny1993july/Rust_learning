# Variable
By the default variables are **immutable**.
If the variable is reassign the without mut variable below error occurs:
```
Compiling Variable_3 v0.1.0 (/home/kamaljeet-singh/Desktop/Programming/Rust/Learning/Variable_3)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {x}.");
4 |     x = 65;
  |     ^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `Variable_3` (bin "Variable_3") due to 1 previous error
``` 

### **mut** in the front of the variable to make it mutable.

# Constant
It is immutable and once set it cannot be changed, not even with mut .
It can be set in any scope even global.
It cannot be any value of an expression.
In Rust, the constant is written in the all caps with underscore between the words.
It is given by given below way:
``` 
const TEST: &str = "Test"
``` 

# Shadowing

By making a variable as it own input to **Overshadow** older variable set using **let** within the scape.
If mut is used then we cannot shadow it.


