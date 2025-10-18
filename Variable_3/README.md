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


