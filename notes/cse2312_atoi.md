(This is written in C, not assembly)

atoi functions
===

atoi stands for **A**SCII **to** **I**nteger, and is the standard function in C
for converting a `char*` to an `int`. Building one might not be inherently
obvious to someone who hasn't done it before, but it really isn't so bad.
(Especially since this won't actually have any checking beyond "is this string
over?")

1. Convert single digit characters
---

The simplest case would be a single digit, for instance a `char[]` of
`['5', '\0']`.

To convert a single ascii digit, we just need to remove the offset of the value
of `'0'` from our digit. (so '0'->0, '1'->1, ... '9'->9)
```c
int atoi( char* input )
{
  return input[ 0 ] - '0';
}
```
This function works correctly for a single digit (note that it doesn't check if
`input[0]` is even a digit).

2. Converting 2 digits
---

To convert two digits at a time, we'll just assume that the string has at least
two characters, at positions `input[0]` and `input[1]`, and both are digits.

Before, we didn't have to deal with place values at all, but now we'll have to
address them:  
The value `21` is most definitely distinct from `2` and `1` individually, so
how can we mathematically compose these to form `21`? By shifting the `2` over
one spot, then adding the `1`! But how do we mathematically represent that? This
is where place value comes in.  
`21` is the same as `20 + 1`, which is the same as `2 * 10 + 1`, and that `* 10`
is the key to the problem. To shift the `2` over, we simply multiply by `10`,
because the `2` is in the `10's` spot.


Knowing this, the resulting code would look like this:
```c
int atoi( char* input )
{
  return ( input[ 0 ] - '0' ) * 10
       + ( input[ 1 ] - '0' );
}
```
We convert the first digit character into a number, then shift it over one place
value, then we convert the second digit character and simply add it. Nothing too
complicated.


3. Converting `n` digits
---

Now, let's try to convert the first `n` digits of a string to an integer. We'll
make the assumption that the string has at least `n` digits in it.

```c
int atoi( char* input, int n )
{
```

The ideas from working with 2 digits sure was nice, but we can't just count the
characters in the string and write a separate way to handle each and every number
of digits (well, you could, but you *really* shouldn't).

So we're going to have to have a loop to handle this instead. And, we'll also 
have to have a number that we build up with the correct answer throughout the loop.
```c
  int result = 0;
  char current;
```

We should just have a standard for loop header for running `n` times
```c
  for ( int i = 0; i < n; i++ )
  {
    current = input[ i ];
```

Next, let's convert the current character into a digit, just as we did before:
```c
    int digit = current - '0';
```

And, in the same way that `21` = `( 2 * 10 ) + 1`, we can rewrite any `n`-digit
number. For example, let's take `n = 4` with the input `4321`:  
```
4321 = ( 4 * 1000 ) + ( 3 * 100 ) + ( 2 * 10 ) + 1
     = 10 * ( ( 4 * 100 ) + ( 3 * 10 ) + 2 ) + 1
     = 10 * ( 10 * ( 4 * 10 + 3 ) + 2 ) + 1
       or more simply
     = ( ( ( ( 4 * 10 ) + 3 ) * 10 ) + 2 ) * 10 + 1
```
Now, this doesn't seem too useful, but it reveals something about how we can go
about solving the problem: We can continually multiplying the previous number by
another `10` before adding another digit, and it'll still work out fine.

So, to append another digit on, we can do this:
```c
    result *= 10;
    result += digit;
```

And that's the end of our loop! Let's just close out those braces and add the
return and we'll be done.
```c
  }
  
  return result;
}
```

The resulting `atoi` function:
```c
int atoi( char* input, int n )
{
  int result = 0;
  char current;
  for ( int i = 0; i < n; i++ )
  {
    current = input[ i ];
    int digit = current - '0';
    result *= 10;
    result += digit;
  }
  return result;
}
```

4. Removing the `n` parameter
---

Now really, we don't need the `n` parameter telling us the length of the
string if it's terminated by a known character (in this case `\0`, but it could
be any character). All it takes is a change to our loop header.

If we know the string is terminated by a `\0`, then we'll just break when
`current = '\0'`, and we can get rid of the loop counter entirely!

To move onto the next element, we just need to move `input` one position forward,
which we can do like `++input`. Then, to get the element at that position, we can
tack on a `[0]`, and get `++input[0]` we will do what we want.

Our new loop header is:
```c
  for ( current = input[ 0 ]; current != '\0'; current = ++input[ 0 ] )
```

And the finished `atoi` is like so:
```c
int atoi( char* input )
{
  int result = 0;
  for ( char current = input[ 0 ];
        current != '\0';
        current = ++input[ 0 ] )
  {
    int digit = current - '0';
    result *= 10;
    result += digit;
  }

  return result;
}
```

