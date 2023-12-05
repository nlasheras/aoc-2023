# aoc-2023
Advent of Code 2023

### Day 1

Second year using Rust, so this year the setup was a bit easier. Haven't done much with it in a long while but I managed to refresh my memory and solve part 1 quite easily with only a small battle against the `&str` and `String` types. For part two, I was surprised that it was a bit of a jump on difficulty so early, but my input didn't seem to contain any tricky edge cases or my assumptions were right from the start.

### Day 2

Second day solution was pretty easy in both parts, but it was relatively work intensive to parse the input into a `struct` but I think the work was worth it, since then writing the solutions was much easier. 

### Day 3

Today the first part was quite easy, since I had already the `Grid` helper I was using last year. After seeing part 2, I reworked my solution a bit to get extra data for every `Part` so I could easily check the adjacent ones for every gear (initially I only returned a list of part numbers, but I changed to return also the number and the starting cell). 

### Day 4

Forgot to write the log. Was a bit trickier to parse than to solve. For part 2 I had to do a slight rewrite but nothing big since it's still a smallish solution.

### Day 5

Every day I'm getting more used to parse the input. Today still has some work to do tomorrow (Independence Day), since I just relied on brute forcing part 2, but I'm pretty sure there is something smart to do. 