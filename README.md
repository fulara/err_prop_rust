# err_prop [![Build Status](https://travis-ci.org/fulara/err_prop_rust.svg?branch=master)](https://travis-ci.org/fulara/err_prop_rust)[![Build Status](https://img.shields.io/crates/v/err_prop.svg?branch=master)](https://crates.io/crates/err_prop)
Super simple ( and stupid ) floating point type calculating upper bound of  error propagation
I dont guarantee that this is correct. If you know you can do something better just contribute! :)  
# how it works
This works on premise that each floating point operation has an error. Where Addition and Subtraction generate actual error and Multiplication Division just increase that error: snapshot of implementation:  
Add:
```
val: self.val + rhs.val,
err: self.val.abs().max(rhs.val.abs()) + self.err + rhs.err
```
Sub:
```
val: self.val - rhs.val,
err: self.val.abs().max(rhs.val.abs()) + self.err + rhs.err
```
Mul:
```
val: self.val * rhs.val,
err: self.val.abs() * rhs.err + rhs.val.abs() * self.err
```
Div:
```
val: self.val / rhs.val,
err: self.err / rhs.val.abs() + rhs.err * self.val / (rhs.val * rhs.val)
```
After you are done with your calculations use the `.err()` method or better `.err_times_eps()` to get static upper bound value of accumulated error
# bunch of method unimplemented!()
There is bunch of methods which are unimplemented!(). Why? Because I needed to provide them however I did not needed them yet, so I just put unimplemented!().  
Most important std::ops::* are implemented that is:  
```
+, - , *, /
```
# why cgmath dependency?
Well. Because I am using cgmath library. and unfortunatelly to be able to use custom num type with cgmath you have to depend on them. :( Because they provide their own trait which has to be implemented.
