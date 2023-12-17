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
│     │                                 │  1   │           55488 │    235.70 μs │
├  1  ┼ Trebuchet?!                     ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │           55614 │      1.84 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │            2720 │    250.70 μs │
├  2  ┼ Cube Conundrum                  ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │           71535 │    228.40 μs │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │          525119 │     67.19 ms │
├  3  ┼ Gear Ratios                     ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │        76504829 │      5.05 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │           22488 │    293.60 μs │
├  4  ┼ Scratchcards                    ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │         7013204 │    332.70 μs │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │         3374647 │      2.31 ms │
├  5  ┼ If You Give A Seed A Fertilizer ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │         6082852 │    922.86 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │          781200 │     13.60 μs │
├  6  ┼ Wait For It                     ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │        49240091 │      3.42 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │       248569531 │      2.50 ms │
├  7  ┼ Camel Cards                     ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │       250382098 │      2.40 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │           24253 │      5.76 ms │
├  8  ┼ Haunted Wasteland               ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │  12357789728873 │     22.85 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │      1974232246 │      1.22 ms │
├  9  ┼ Mirage Maintenance              ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │             928 │      1.18 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │            6768 │      3.12 ms │
├ 10  ┼ Pipe Maze                       ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │             351 │      3.94 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │        10313550 │     22.93 ms │
├ 11  ┼ Cosmic Expansion                ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │    611998089572 │     25.69 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │            7670 │     12.79 ms │
├ 12  ┼ Hot Springs                     ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │ 157383940585037 │    540.87 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │           34772 │    578.80 μs │
├ 13  ┼ Point of Incidence              ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │           35554 │     61.51 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │          108614 │    184.40 μs │
├ 14  ┼ Parabolic Reflector Dish        ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │           96447 │     69.40 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │          503154 │    293.60 μs │
├ 15  ┼ Lens Library                    ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │          251353 │      1.44 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │            7415 │    176.80 μs │
├ 16  ┼ The Floor Will Be Lava          ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │            7943 │     13.55 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│     │                                 │  1   │            1001 │     25.68 ms │
├ 17  ┼ Clumsy Crucible                 ┼──────┼─────────────────┼──────────────┤
│     │                                 │  2   │            1197 │     40.74 ms │
├─────┼─────────────────────────────────┼──────┼─────────────────┼──────────────┤
│                                    17 / 24                                    │
└─────┴─────────────────────────────────┴──────┴─────────────────┴──────────────┘
```
