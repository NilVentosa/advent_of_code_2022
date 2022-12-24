# advent_of_code_2022

Skeleton builder for aoc. To run it run:

`cargo run -- -d ${day_number}`

This will generate a rust project named `day${day_number}`, download the input
for that day into `dayX/input.txt` and add a `src/main.rs` template. It will also
generate a empty `test_input.txt` file.

You need to set the env variable `AOC_COOKIE` to be able to download the input,
you can find it inspecting a request from the browser. The value should be like:
`session=xxxxxxxxxx...`

In this repo also my solutions to the advent of code 2022 to learn some rust.
