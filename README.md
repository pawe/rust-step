Parsing STEP files in Rust

Rust is of interest to me and I want to learn more about it. Let's implement a STEP parser in it. Please be aware, that this is a learning project.

STEP is a file format to exchange … 
It’s defined in ISO 10303-21 standard, which costs money, but luckily draft documents are available at http://www.steptools.com/stds/step/IS_final_p21e3.html

Use a combinatory parser library called nom (http://). Small pieces are put together to build more complex parsers.

“Parsers are usually built from the bottom up, by first writing parsers for the smallest elements, then assembling them in more complex parsers by using combinators.”



