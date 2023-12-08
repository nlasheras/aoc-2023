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

### Day 6

I had some plans for the day, since it was a holiday in Finland so I tried to be efficient with my time. I started with a straight forward implementation to get the higher and lower bounds and that worked pretty well for both parts of the problem. Still, after getting the stars I had some time so I got pen and paper and just did the math and implemented the solution using the equation. I tried to use `f64::EPSILON` but the rounding didn't work so I just took it easy and used a *0.0001* for the edge case that the solution is round number.

### Day 7

Didn't have time in the morning for this one. My logic to get the groups of matches may not be the nicest, but was pretty stright forward to get all the options. Then it was just figuring out how to use `sorted_by` from itertools worked to get the hands sorted properly. For part 2 I was able to get the answer by hacking my code pretty fast, to get the final version, I had to learn how to pass functions as parameters, so I could make my rank have an extra parameter to use the *jokers*. 

### Day 8

Pretty straight-forward day. For part 2 I started reworking the solution to accept many inputs, but when running it with the input saw that it seemed to take a bit. Without thinking too much I realized the trick and that was pretty easy to implement since the `num` crate provides the functionality. 
