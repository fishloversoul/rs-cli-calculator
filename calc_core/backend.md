# Backend

The backend take in a string input and returns a Result f32, String 

### Eval 
Is just for having a clean opening to the outside and to connect the other functions to each other.

### Tokeniser 
First it takes a &str then it trims turns to char and filters out all the white spaces and collects it into a string then i loops over every char in the string if the char is a number between 0 - 9 it saves to a current then if it a operator (*/-+^()) then it check if it a  - symbol and if is_number true then it will push the - symbol to current (this is for negative numbers), if that false then it check if the current is empty and if not it pushes current to output the it check if c ( the character) != “)” ( this is so that after “)” it won’t make the next number a negative instead it makes it a minus symbol) then it pushes c to output and it does that for all the chars in the string after it send back the output ( the Vec<String>). 

### Parentheses remover
perent_t is used to get rid of all the parentheses it does this by looping over everything in Vec<string> and it checks if theres  “(“ when it hits that it loops over until it finds the closing “)” it does this by check if it hit “(“ it adds ones to depth and when it hits “)” it does - 1 until it hits 0 then it take the place of the starting one and the ending one copies it into another vec and send it back into perent_t which loops everything over again until there's no more “(“, then it sends to red_ops_do just calls red_ops for every operation in pemdas order red_ops just collapses operations like 10 - 10 into just the answer then it removes form the original Vec all the thing inside the () to just the answer.

### Red_ops_do
This just calls Red_ops in pemdas order.

### Red_ops
This does the collapsing of operations like 10 - 10 into just the answer. It does this by taking the Vec and the symbols to look for. It loops over everything in the Vec when it finds one of the symbols it grabs the 2 number around it then it does a match and does the operations on the number and replaces the first one with the answer and removes the other 2.
