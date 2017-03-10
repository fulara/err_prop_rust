# err_prop [![Build Status](https://travis-ci.org/fulara/err_prop_rust.svg?branch=master)](https://travis-ci.org/fulara/err_prop_rust)
Super simple ( and stupid ) floating point error propagation calculating type  
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
# bunch of method unimplemented!()
There is bunch of methods which are unimplemented!(). Why? Because I needed to provide them however I did not them yet, so I just put unimplemented!().  
Most important std::ops::* are implemented that is:  
```
+, - , *, /
```
# why cgmath dependency?
Well. Because I am using cgmath library. and unfortunatelly to be able to use custom num type with cgmath you have to depend on them. :( Because they provide their own trait which has to be implemented.
