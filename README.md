# README

## 1. The problem

Company’s stakeholders want to get a list of all BAR token (ERC20) transfers where the from OR the to is part of a given list and have happened over the last X years.
Write a code snippet in your preferred language without using a 3rd party provider (like Alchemy, Moralis, ...).
Then describe how you could improve your code by using a modern approach and keeping in mind security, scalability, maintainability, speed, use of 3rd party tools, etc..

## 2. The solution: A naive approach
The solution is based on Rust with `1.75.0` version.

The code snippet is contained in `src/main.rs`.

The solution presented is a straightforward and naive implementation that can solve the problem.

Of course, there are many way to improve that solution that will be described later.

The naive approach is based on a while loop that starts inspecting the latest block in chain and proceed in reverse order until the block timestamp is after a certain amount of time defined in the code.
For simplicity the interaction with the Ethereum Sepolia testnet happens via Alchemy.

The main drawbacks of this solution are:

- It's slow. If the number of years increase the time spent executing the logic increase in a linear way.
- It's not memory efficient.

### 2.1 What can be improved: Speed

A way better approach would be taking advantage of parallel computation and introduce threads.

This kinds of problems (collecting data and calculate results) can be brillantly solved with a map-reduce algorithm.

In such case the program would do the following:

- Find the first block on chain that has a timestamp contained in the selected range of inspection.
- Split the blocks numbers in chunks.
- Create a thread pool where evry thread implements the "map" phase:
  - Inspects the blocks numbers and search for matching transactions.
  - Returns the list of matching transactions.
- Implements the "reduce" phase where all the results are collated together.

### 2.2 What can be improved: Use of third-party services

There are thrid party services (like Alchemy) that provides ad-hoc REST APIs for solving the same problem. Even thought they are convenient from a development perspective and can cut time the businees should be aware that:

- Third-party services have SLA, hence there can be downtime. Depending on how important is getting this info in a reliable and predictable time slot a third-party service can be discarded in favour of owning/managing a node.
- Usually it's a paid service.

### 2.2 What can be improved: Scalability

The naive solution is intended to be a service running at scheduled times and/or for limited cases. If the requirements become monitoring token transfers at scale across multiple smart contracts and users the solution should be completely refactored and the logic would be completely different. For example, it is possible to think of a monitoring application that consistently inspects the blocks added to the chain and keeps track of the transfers that happen storing them on a persistent storage like a database.

## How to run it

To run the application do the following step:

1. Install Rust (see [How to install Rust](https://www.rust-lang.org/tools/install))
2. Add an API key to use Alchemy
3. Move to the root folder of this repository and execute `cargo run`