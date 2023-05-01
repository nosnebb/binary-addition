# binary-addition
just a rewrite of my computer science projec that I had on time in another language to practice

## Description
Given a text file containing binary numbers two on each line seperated by a space; such as, "1101 0110" do a simple binary addition of the two numbers.
Then output the result to standard output. Usually this is the terminal, but the command should handle the output being piped into another command.

## Requirements
The implementation should be able create multiple processes. The first process is responsible for reading in every distinct line. Accounting for the various line endings that can occur accross multiple operating systems. The second process should handle the binary addition of the now read string. Finally, the last process should output the resulting addition to standard output. Each of these processes shall communicate with one another using a 

## Bonus Points
Additional features to include are the option to output a verbose argument to the screen. This cli command should be common sense; such as, "./myprogram -v". Finally, once the program meets these requirements. A second argument may be supplied to the program to denote whether to use threads, or processes to handle the different scenarios presented in the aforemented base requirements. This means that instead of using invidual processes to handle the "work" of binary addition three threads will be created to handle the input and output of the program.
