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
val input: String = ...
var output = ArrayList< Byte >()
for ( i in 0..input.lastIndex step 2 )
{
  output +=
      ( input[ i ] - '0' ).shl( 4 ).toByte()
    + ( input[ i + 1 ] - '0' ).toByte()
}
```
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
val input: ArrayList< Byte > = ...
var output = ""
for ( byte in input )
{
  val firstNibble = ( byte.toInt() and 0xF0 ) shr 4
  val firstASCII = ( firstNibble + '0'.toByte() ).toChar()
  output += firstASCII

  val secondNibble = byte.toInt() and 0x0F
  val secondASCII = ( secondNibble + '0'.toByte() ).toChar()
  output += secondASCII
}
```
* `shr` is the right bitshift, `>>`
* `and` is the bitwise and, `&`

