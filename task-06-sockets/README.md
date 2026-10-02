# DD1337 Week 6

Modified: Dmitry Chirin (Inspired by ++22 assignment by Anton Lilja and Morris Hansing)

## The Internet is for... Fun!

This week we'll be working on implementing networking for the chess libraries.

To get started, see the Rust book's [multithreaded server example project](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) (or look at the following examples down bellow).

Tobias Hansson has graciously provided some of my old Java code under `./java-examples` a server and client program. Even though we're going to be working in Rust, it's a simple and good example you can take some inspiration from.

Viola Söderlund has touched up a very simple Rust server and a client under `./rust-example`. Check it out!

Dmitry Chirin added another even simpler version of sockets in Rust.

### Prepare your assignment

1. Create a repository named `<KTH_ID>-task-6`
2. Start adding networking to a chess gui (your own, or someone elses).

## Assignment

### Improve your chess-gui!

Add multiplayer functionality to your chess-guis (or someone elses).

Grading:
* Getting a pass on these plus assignments does not require a perfect solution, but some time invested and a reasonable attempt is expected.
  * To be more specific, some kind of communication between two clients (most likely a client and a server) must be established, and some data must be sent and received between the two.
* It is always possible to do the week's regular assignment instead but beware that the grading is much stricter in that case. Notify your asse/TA if you are doing the regular assignment so he/she knows what to grade.