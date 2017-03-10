# err_prop [![Build Status](https://travis-ci.org/fulara/err_prop_rust.svg?branch=master)](https://travis-ci.org/fulara/err_prop_rust)
Super simple ( and stupid ) floating point error propagation calculating type  
I dont guarantee that this is correct. If you know you can do something better just contribute! :)  
# bunch of method unimplemented!()
There is bunch of methods which are unimplemented!(). Why? Because I needed to provide them however I did not them yet, so I just put unimplemented!().
# why cgmath dependency?
Well. Because I am using cgmath library. and unfortunatelly to be able to use custom num type with cgmath you have to depend on them. :( Because they provide their own trait which has to be implemented.
