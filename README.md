# Rs-cli-calculator 

cli rust calculator that flows pemdas and can do operations with ( + - * / ^ () ).
The backend of the calculator is written in no_std (it doesn't use the stander library of rust

## Frontend

The frontend only reads and writes to the terminal

## Backend

The backend takes a string, tokenises it into a Vector, gets rid of parentheses and then does the operations. Then returns the answer all the way back up and also has error handling built in so if something goes wrong then it won’t crash and would return an error message.

### Learned

- Taking input
- Tokenising input
- Error handling
- Recursion 


The backend can run on any system if you port it over. It only about 1.17 mb for the entire thing on a windows exe that includes the frontend and all the thing to make it run on windows if you got ride of the extra stuff it could come down to about 0.5 mb. The backend is only 147 lines of code. Currently does not support exponent power of floats it can do base floats. I will work later on updating it to support exponent power of floats.


[backend deep dive](https://github.com/fishloversoul/rs-cli-calculator/tree/main/calc_core)
