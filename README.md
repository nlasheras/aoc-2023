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

### Day 5 :warning:

Every day I'm getting more used to parse the input. Today still has some work to do tomorrow (Independence Day), since I just relied on brute forcing part 2, but I'm pretty sure there is something smart to do. 

### Day 6

I had some plans for the day, since it was a holiday in Finland so I tried to be efficient with my time. I started with a straight forward implementation to get the higher and lower bounds and that worked pretty well for both parts of the problem. Still, after getting the stars I had some time so I got pen and paper and just did the math and implemented the solution using the equation. I tried to use `f64::EPSILON` but the rounding didn't work so I just took it easy and used a *0.0001* for the edge case that the solution is round number.

### Day 7

Didn't have time in the morning for this one. My logic to get the groups of matches may not be the nicest, but was pretty stright forward to get all the options. Then it was just figuring out how to use `sorted_by` from itertools worked to get the hands sorted properly. For part 2 I was able to get the answer by hacking my code pretty fast, to get the final version, I had to learn how to pass functions as parameters, so I could make my rank have an extra parameter to use the *jokers*. 

### Day 8

Pretty straight-forward day. For part 2 I started reworking the solution to accept many inputs, but when running it with the input saw that it seemed to take a bit. Without thinking too much I realized the trick and that was pretty easy to implement since the `num` crate provides the functionality. 

### Day 9

Today's puzzle was pretty easy to implement in Rust. The small research I did for this was to figure out what was the best way to concatenate two vectors. For the part 2 I refactored the solution to return the whole array, but since in the end only one value matter I could have probably ignored that.

### Day 10

This one was tricky! First part was pretty easy since it was just finding the shortest path for every point of the loop (even though we were interested in the longest one). Part 2 I had to research a bit until I found about the *even-odd rule* which seemed to work for the example and the input.

### Day 11

When reading the problem I was starting to worry about having to use A* to check for the shortest path because I was expecting the galaxy expansion to be unmanageable (*lanternfish* scale), at least in part 2. After I did the expansion of just the positions I realized that the shortest path was trivial to compute, so the rest was pretty easy. 

### Day 13 :warning:

I was a bit busy on Wednesday, so I skipped the day and did it on Thursday. Since I have 12.2 to finnish and 14 to do I just went with the straight forward solution and I'll take another look to the solution once I have more time.

### Day 14

Part two sounded familiar to a previous one I made in a previous year, so I just used the same cycle searching trick (I think I'll move that to an utility function because I think it's handy to have for next years).

### Day 15

Part one was pretty fast to do. Part two was also pretty fast to do, I was a bit afraid that I would stumble a bit against the borrow checker, but it seems that after 15 days of Rust I have started to use the right idioms. 
