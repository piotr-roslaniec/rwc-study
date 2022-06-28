# exercises-01


## Ex 01-01 

_MD5 is said to be ‘insecure/broken’. Which security properties are vulnerable? Prove it._

MD5 generates 128-bit digest which gives us 64-bit security. It means that an attacker has to perform 2^{64} operations to have 50% of chance to break the hash and recover the original input.

In order to have reasonable security guarantee we have to assume that we need at least 128-bit of output. That assumption would make MD5 insecure for collision-resistance (which requires 256-bit output), pre-image resistance (128-bit), and second pre-image (also 128-bit)  

## Ex 01-02 

_Calculate the total theoretical number of attempts it would take to brute force various hashes digests (MD5, SHA-1, SHA256)._

- MD5: 128-bit output, 64-bit security, 2^{64} operations
- SHA-1: 160-bit output, 80-bit security, 2^{80} operations
- SHA256: 256-bit output, 128-bit security, 2^{128} operations

## Ex 01-03

_Find a digest collision of the first 4/6 bits of any two input string MD5 hash digests._


```
Original input: [6f, 72, 69, 67, 69, 6e, 61, 6c, 5f, 69, 6e, 70, 75, 74]
Original digest: [c2, b6, c9, 8d, e8, 9a, d4, 20, 93, c9, be, 55, 67, 75, a4, 85]
Bytes for collision: 4
Collision found: [c2, b6, c9, 8d, ba, 61, cd, ce, 31, 35, 3f, 41, dc, 55, 21, 34]
Hash input: [be, 35, c2, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
Attempts: 146945470
```

See `./ex03` for details

## Ex 01-04

_Missing_

## Ex 01-05

_Missing_

## Ex 01-04

_Explain and demonstrate how to calculates the Hamming Distance between two strings._

Hamming distance is the number of bit positions at which two strings differ.
Let's calculate Hamming distance using this example:

- 0 0 1 1
- 0 1 0 1

We can see that the strings have different bit positions at indexes 1 and 2

- 0 **0 1** 1
- 0 **1 0** 1

So the Hamming distance between these two strings equals 2

## Ex 01-05

_What is the Hamming Distance between any bytestring hashes where i1 (unmodified) and i2 has 1 bit flipped._

It equals one, becasue the string differes only at one position (one bit).

## Ex 01-06

_Explain and demonstrate the difference b/w Second Pre-Image Resistance and Collision Resistence._

Second Pre-Image Resistance and Collision Resistence are both security properties of hash functions. 


Second Pre-Image Resistance describes a situation, where an attacker attempts to reverse input from a hash function output. This security property relies on how well the hash function implementation is "scrambling" it's input. The more random the output, the better.

The second one describes a situation, where an attacker attempts to find two inputs that result in the same hash function output. This security property relies on the size of the output. If the output size is small then it's easier to enumerate and recover


## Ex 01-07

_Explain and demonstrate the calculation of ‘The Birthday Bound’ Paradox._

The Birthday Paradox present an non-intuitive scenario where in a rooom of 23 people there's a 50% chance of two of the having the same birithday. In a room of 75 people there's a 99.% chance of two people matching.

In order to properly calculate those probabilities we have to develop an intuition about what are we really counting here.

We have a room of 23 people and we want to figure out whether any given pair of people there may have a birthday on the same day. So we have to calculate the number of all possible pairs of people.

(23 * 22) / 2 = 253

Now, for just a single pair of people, what is the probability that they have birthday on the same day? Well, there's just one day in the year where both of day could have their birthdays, so that's 1/365.

1/365 is pretty bad odds for a single pair, but let's remember that we're going to bet 253 times, over all possible pairs of people.

So what now, do we just multiply these two? 

1/365 * 253 = 253/365

But what sense does it make? Well, no. We're misunderstanding how probabilities work. 

Let's take a coin toss for an example. When you toss a coin, have 50% chance for each heads and tails. To get heads twice, you have 25% chance. So you may think it's 1/2 * 1/2, and you would be right - but it's not about multiplication (or division). It's about the exponent: 0.5^{2}

So our problem may be rewritten as:

(1/365)^{253}

But that would give us an incredibly small value! Aren't we forgetting something?

If we were to state our calculation like this, that would mean we are betting on having a winning pair every time, just as we would be asking to have heads or tails landing every time:

(1/2)^{253}

What we're really betting on is a situation where across all of these pairs we would at least one positive outcome

(1 - 1/365)^{253} ~ 0.4995

And that's pretty close to 50%!


## Ex 01-08

_Find an input string which results in a SHA256 hash with 1/2/X 0’s (zero)_

## Ex 01-09

_Find X (look up, don’t over think it): md5(X).digest() > d41d8cd98f00b204e9800998ecf8427e_

## Ex 01-10

_Prepare an exercise related to XOR bitwise operations (compress/uncompress)_

## Ex 01-11

_Prepare an exercise related to serialization / deserialization_

## Ex 01-12

_Explain and demonstrate the difference between cryptographic hash functions and checksum functions (CRC32)_