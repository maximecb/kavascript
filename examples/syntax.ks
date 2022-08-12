// The let statement lets us declare variables
let x = 1;
let x2 = "foo";

// Assert allows us to check that an expression evaluates to true
// Only the integer 0 evaluates to false
assert x;

// Println prints values and then a newline
println(1);
println(2);
println(1,2,3);
println(x2);
println("Hello world!");

// While loops are supported
let i = 0;
while (i != 10) {
    println("i = ", i);
    i = i + 1;
}

// There are if and else statements
if (0 < 10)
    println("true");
else
    println("false");
