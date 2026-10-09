# DD1337 Week 7

Heavily edited by: Benjamin Widman

## Getting started with C

### Install GCC toolchain
> Note: This is already installed in most Linux distros, so if you are using something like Ubuntu (hint to WSL users) you can skip this.
1) Install [MingW](https://www.mingw-w64.org/). *Windows-users are recommended to just use WSL*.
2) Test your installation by entering `gcc --help` in your terminal or command prompt(*host pleb *host).

Congrats dear programmer. You can now compile your C files by entering the following command. (`$` indicates a terminal prompt)
```
$ gcc program.c -o name_of_executable
```
To run:
```
$ ./name_of_executable
```

### Prepare for your assignment

1) Create a repository named `<KTH_ID>-task-7`.
2) Clone your newly created repository.
3) Inside a `assignment-2` folder, create one `.c` file per subassignment. Name then descriptively.
4) copy over the `assignment-3` folder into your newly created repository.

For help with code setup, begin by copying the contents of `./kattis_template/main.c` into your `.c` file.

If you're not familiar with the C language I recommend reading up on it online, for example [GeeksForGekks](https://www.geeksforgeeks.org/c-programming-language/).

## Assignment 1 - Kattis

As a warmup, solve [A Different Problem](https://open.kattis.com/problems/different), which is now on Open Kattis and not KTH's Kattis (i.e. not locked behind courses😄).

See `./minimal_scalar_product` for a Kattis solution example in C.

**Don't forget** to include screenshots of Kattis in the README.md to prove that your solutions work. (like in the [README-template](https://github.com/IndaPlus26/Rustoholics-Instructions/blob/main/README-template.md))

## Assignment 2.1 - RSA key generation

The main part of this task will be to implement the [RSA public-key cryptosystem](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) to encrypt files with a public encryption key such that they're unreadable, and incredibly difficult to crack, without the private decryption key. The first, and biggest, part will be to create these two keys.

Create a program called `keygen.c` which creates one file containing the public key and another containing the private key. E.g. `public.key` and `private.key`.

- To understand RSA, watch this short video: https://www.youtube.com/watch?v=wcbH4t5SJpg
- Or Eddie Woo's two videos:
    - Intro: https://www.youtube.com/watch?v=4zahvcJ9glg
    - Generating keys: https://www.youtube.com/watch?v=oOcTVTpUsPQ

So the important values we have are:

| Value | Description |
| --- | --- |
| p = large prime | Random, but p != q. |
| q = large prime | Random, but p != q. |
| n = p * q | The modulo. Public, but it's incredibly hard to find the two prime factors. |
| r = (p-1)(q-1) | Modulo to find d. Actually Euler's phi function of n, but don't mind that until diskmatten. |
| e = 7 | The encryption exponent. Public. Can be other bigger prime numbers but we keep it small to not run into overflow. |
| d = e^-1 (mod r) | The decryption exponent. Private. |

The public key is `(e, n)` while the private key is `(d, n)` (n is included in the private key here for simplicity). To make the algorithms cross compatible between students we will all be using the same format for the keys and encrypted messages. Now you can finally talk shit with eachother without us being able to read it (as long as one of the primes isn't 67 >:( ) 

### Key formating

#### `private.key`:
```
-----BEGIN RSA PRIVATE KEY-----
d
n
-----END RSA PRIVATE KEY-----
```

#### `public.key`:
```
-----BEGIN RSA PUBLIC KEY-----
e
n
-----END RSA PUBLIC KEY-----
```

There are multiple ways you can find prime numbers. One example is the [sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) (note there's pseudo code for it) or a faster but much harder to implement [sieve of Atkin](https://en.wikipedia.org/wiki/Sieve_of_Atkin).

When selecting two random prime numbers out of the ones you've found with a sieve, make sure you initialize rand with the current time as the seed to avoid always getting the same output when running the program:

```c
#include <time.h>
#include <stdlib.h>

int main() {
    srand(time(NULL));
    int p = rand();
    int q = rand();

    return 0;
}
```

For simplicity sake I suggest limiting the value of `n` (thus how big the p & q you choose are) to fit within a C datatype, like `int` or better `unsigned long long int` (nothing some evil type casting can't handle muahaha). Using prime numbers around the range 500-1000 *should* not cause any overflow, but please test around. However, if you want to you can extend your key size to 2048 or 4096 and in that case increase `e` to 65537, for top-class security.

> Note: `e^-1` is not `1/e` but the inverse of `e` in modulo `r`. If you remember from basmatten it's the number which when multiplied with `e` equals 1 in modulo `r`. I.e: ed (mod r) = 1.

`d` can either be found using the [extended Euclidean algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm) or more simply looping and testing every multiple of `e` to see if it is equal to 1 in modulo r.

## Assignment 2.2 - RSA encryption

Create a program called `encrypt.c` which takes the name of a text file as a command line argument and encrypts the content into a new file prefixed with "encrypted": `encrypted_<previous name>.msg`.
```
$ ./encrypt text.msg 
```
Which outputs: `encrypted_text.msg`

It's supposed to read in the public key `(e, n)` from the previously generated file and then encrypt using: `m^e (mod n)`

`m` is the message we want to encrypt. When we are encrypting a string of text we need to run this for every character or byte.

### Encypted message format
#### `encrypted.msg`:
```
-----BEGIN ENCRYPTED MESSAGE-----
encrypted char1
encrypted char2
encrypted char3
encrypted char4
...
-----END ENCRYPTED MESSAGE-----
```

### How to get CLI arguments

Command line arguments are retrieved from `argv` when using the following function header:
```c
int main(int argc, char **argv) {}
```
- `int argc` is the argument count, i.e. the number of arguments supplied. This is useful for either error checking that the correct number of arguments where supplied or looping through the arguments when there can be a varied amount of them.
- `char **argv` are the argument values and is a C-style string array since `char*` is a string and arrays are just pointers, hence the double pointer.

## Assignment 2.3 - RSA decryption

Create a new program called `decrypt.c` which takes the same command line arguments as `encrypt.c` but instead reverses the encryption of the supplied text file. The result shall be in a new file with either the prefix "encrypted" removed or with a new prefix "decrypted" for fun growing names🤪.

It reads in the private key (d, n) from the previously generated file and then decrypts using: `c^d (mod n)`

`c` is the cipher text, i.e. the encrypted message.

## Assignment 3 - Creating and Cracking secret messages

You may have noticed there are 5 files inside the assignment-3 folder: 

1) `public.key` - Contains the public key that should be used to encrypt a message for me.

2) `for_ta.msg` - Should contain the encrypted message you just encrypted.

3) `private.key` - Contains the private key that should be used to decrypt the `encrypted.msg`.

4) `encrypted.msg` - The encrypted message that you need to decrypt.

5) `decrypted_proof.msg` - After decrypting the `encrypted.msg`, you should put the now readable message inside here.

So in short for this assignment you need to encrypt a message for us using the `public.key` and put the result inside `for_ta.msg`. You also need to decrypt the message inside `encrypted.msg` using the `private.key` and put the result inside `decrypted_proof.msg`.
