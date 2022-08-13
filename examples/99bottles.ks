let n = 99;

while (n > 0)
{
    let plural = "";
    if (n != 1)
        plural = "s";

    println(n, " bottle", plural, " of beer on the wall, ", n, " bottle", plural, " of beer.");

    n = n - 1;

    if (n != 1)
        plural = "s";
    else
        plural = "";

    println("Take one down and pass it around, ", n, " bottle", plural, " of beer on the wall.");
}

println('No more bottles of beer on the wall, no more bottles of beer.');
println('Go to the store and buy some more, 99 bottles of beer on the wall.');
