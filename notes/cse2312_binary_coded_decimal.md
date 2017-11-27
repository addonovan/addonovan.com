Binary Coded Decimals
===

Binary Coded Decimals are the concept of embedding numeric values inside
of bytes, such that they'll take up less space. The idea is that a numeric
string, such as "9583572" would take at least `n` bytes to store as a regular
ASCII sequence, but we can represent the numbers in the range 0-9 (i.e. all
digits) using only 4 bits, so we could halve the number of bytes by packing
the digits together.

Converting to BCD
---

The basic process is:
1. Take the nth value from the array (where n % 2 == 0)
2. Convert this to its numeric value (subtract ASCII value for 0)
3. Shift this value to the left 4 bits
4. Take the (n+1)th value from the array
5. Convert this to its numeric value
6. Add these two together (or use bitwise or, `|`)

This is an example of how to do it in Kotlin.

```kotlin
fun String.toBCD(): ArrayList< Char >
{
  val output = ArrayList< Char >()

  for ( i in 0..this.lastIndex step 2 )
  {
    val firstASCII = this[ i ]
    val firstNumber = ( firstASCII - '0' ).toInt()
    val firstNibble = firstASCII.shl( 4 )

    val secondASCII = this[ i + 1 ]
    val secondNumber = ( secondASCII - '0' ).toInt()
    val secondNibble = secondNumber
  
    output += ( firstNibble + secondNibble )
  }

  return output
}
```
* `this` refers to the numeric string being encoded
* `shl` is the left bitshift, `<<`
* the addition could also be represented by `or`, the bitwise or, `|`

Converting from BCD
---

The process is the reverse of how to convert into it (duh), with no tricks up
its sleeve.

1. Take the nth byte from the byte array
2. Take the first (i.e. most significant 4) bits from it
3. Shift this to the right 4 places
4. Convert this to its ASCII value
5. Take the last (i.e. least significant 4) bits from the byte
6. Convert this to its ASCII value

Kotlin sample:
```kotlin
fun ArrayList< Char >.fromBCD(): String
{
  var output = ""
  for ( char in this )
  {
    val firstNibble = ( byte.toInt() and 0xF0 )
    val firstNumber = firstNibble shr 4
    val firstASCII  = ( firstNumber + '0'.toInt() ).toChar()
    output += firstASCII

    val secondNibble = ( char.toInt() and 0x0F )
    val secondNumber = secondNibble
    val secondASCII  = ( secondNumber + '0'.toInt() ).toChar()
    output += secondASCII
  }

  return output
}
}
```
* `this` refers to the list of characters being decoded
* `shr` is the right bitshift, `>>`
* `and` is the bitwise and, `&`

Example
====

Convert "$a" from a binary coded decimal to a numeric string.
We'll use the hexadecimal values for characters because it separates their values into nibbles.

**$**
1. The ASCII value for `$` is 24 in hex (36 dec)
2. To get the first nibble, we `&` it with `1111 0000` (bin) and get `0010 0000` (bin)
3. Shift this right 4 places to get `0010` (bin)
4. Add value of '0' to get ascii representation ('2')
5. Take second nibble, we `&` it with `0000 1111` (bin) and get `0000 0100` (bin)
6. Add value of '0' to get ascii representation ('4')

So the first two characters of our output are "24"

**a**
1. the ASCII value for `a` is 61 (hex) (97 dec)
2. To get the first nibble, we `&` it with `1111 0000` (bin) and get `0110 0000` (bin)
3. `>> 4` to get `0110` (bin)
4. `+ '0'` for `'6'`
5. `& 0000 1111` (bin) and get `0000 0001` (bin)
6. `+ '0'` for '`1'`

So the entire value of this BCD is "2461"

| Method        | Payload (hex) | Bytes used |
| ------------- | ------------: | ---------- |
| Binary Number |          9 9D | 1.5 (2)    |
| ASCII String  |   32 34 36 31 | 4          |
| BCD           |         24 61 | 2          |

