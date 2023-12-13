# Advent of Code 2023 in Rust

My AoC 2023 solutions in Rust, only if I don't get bored of it before it ends.

![Dabbing santa](https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExbWJ5ZWU4aTdzcXFlZXExdGEwNWJzcTIyMXVvdzQyZmk4cGw0YnQ4MCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/oer4pWGncRFWSzBbKj/giphy.gif)

I'm definitely not a Rust expert, so don't expect anything fancy here and you probably shouldn't use this as a reference for anything.

![Dancing santa](https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExY3pyYnZ3aXJpd3lkMGx5Y295dXYzODNibGdzaG9rcTIxdjd2MHdhciZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/ard2bUzUMewmcXh3N7/giphy.gif)

## Running
Running on my AMD Ryzen 7 3700x, with 32GB of RAM, on Windows 10.
```sh
┌─────┬─────────────────────────────────┬──────┬─────────────────┬──────────────┐
│                           🎄 Advent of Code 2023 🎄                           │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│ Day │ Name                            │ Part │ Answer          │ Compute time │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │           55488 │    228.00 μs │
├  1  ┼ Trebuchet?!                     ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │           55614 │      1.93 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │            2720 │    268.10 μs │
├  2  ┼ Cube Conundrum                  ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │           71535 │    236.50 μs │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │          525119 │     64.53 ms │
├  3  ┼ Gear Ratios                     ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │        76504829 │      5.31 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │           22488 │    294.20 μs │
├  4  ┼ Scratchcards                    ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │         7013204 │    387.40 μs │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │         3374647 │      2.27 ms │
├  5  ┼ If You Give A Seed A Fertilizer ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │         6082852 │    940.51 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │          781200 │      5.70 μs │
├  6  ┼ Wait For It                     ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │        49240091 │      3.39 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │       248569531 │      2.92 ms │
├  7  ┼ Camel Cards                     ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │       250382098 │      2.60 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │           24253 │      6.16 ms │
├  8  ┼ Haunted Wasteland               ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │  12357789728873 │     23.73 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │      1974232246 │      1.30 ms │
├  9  ┼ Mirage Maintenance              ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │             928 │      1.27 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │            6768 │      3.34 ms │
├ 10  ┼ Pipe Maze                       ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │             351 │      4.03 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │        10313550 │     23.49 ms │
├ 11  ┼ Cosmic Expansion                ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │    611998089572 │     26.16 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │            7670 │     12.80 ms │
├ 12  ┼ Hot Springs                     ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │ 157383940585037 │    524.26 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │           34772 │    560.10 μs │
├ 13  ┼ Point of Incidence              ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │           35554 │     63.76 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│                                    13 / 24                                    │
└─────┴─────────────────────────────────┴──────┴─────────────────┴──────────────┘
```
