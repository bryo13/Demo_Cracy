# Cracy
This is an application that simuates an election process in rust.

## Election simulation
- A person would choose their prefered candidate
- Their less prefered candidate and so on until their least prefered candidate
- To try and prevent spoilt votes, the electorate would have to go through all candidates in order in order for the votes to count.
- The total tally of the votes would determine the winner.

## Running
Adding the deps in Cargo.toml in your system should do the trick
i.e
```bash
cargo build
```

## Inspiration
A [Veritasium](https://www.youtube.com/watch?v=qf7ws2DF-zk) youtube video
 on democracy got me thinking a small model simulating an election
 would be a good tool to learn rust.
