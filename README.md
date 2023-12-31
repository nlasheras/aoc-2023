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

### Day 12 :warning:

I completed this pretty late. The part 1 was pretty straightforward, and the brute-force solution was good enough for part 1. For part 2 I had to rewrite a new version of the function in order to use memoization. This one I really need to get back to, because I had to clone the input many times even though is constant because of the `memoize` crate. I need to research if it's possible to use a static or any kind of reference for this. 

### Day 13 :reminder_ribbon:

I was a bit busy on Wednesday, so I skipped the day and did it on Thursday. Since I have 12.2 to finnish and 14 to do I just went with the straight forward solution and I'll take another look to the solution once I have more time.

### Day 14

Part two sounded familiar to a previous one I made in a previous year, so I just used the same cycle searching trick (I think I'll move that to an utility function because I think it's handy to have for next years).

### Day 15

Part one was pretty fast to do. Part two was also pretty fast to do, I was a bit afraid that I would stumble a bit against the borrow checker, but it seems that after 15 days of Rust I have started to use the right idioms. 

### Day 16 :warning:

Today the biggest problem I got was in Part 1, since even with the example my starting implementation was staying on a loop. My starting solution for that was just splitting the beams first time a beam passes over there, which ensures that the algorithm will end eventually. I tried a more correct version of the algorithm afterwards, but my first implementation was much slower.

### Day 17

I got stuck a bit with this problem for a while. I have tried my implementations of A* and Dijkstra but seems that there is something I'm missing on the puzzle. I'll go ahead for now and work on this during the holidays.

I had some time right before the break to work on this one. The challenge of mapping the restrictions onto the problem was the tricky part. I think this is going to be a nice test for moving my A* implementation to some generic utility I can re-use on other problems.

### Day 18

After a couple of days stuck on the previous problem, it was nice to get some easier ones. Being so advanced on the calendar, I discarded the idea of using a grid right away. I guessed that the idea is using math for it, and a quick search for how to compute the area of a polygon given a set of points got me to the right algorithm to get the points inside the boundaries (*Shoelace formula*). For the boundary I noticed that it was half plus one in order to match the example, but then reading the Wikipedia page for Shoelace formula I stumbled into *Pick's theorem* which is exactly that formula. Part 2 was pretty easy after having figured the math version, as it works right away. 

### Day 19

This one was on the laborious side but easy in part 1. For part 2 I more or less had the idea of how to do it, I was a bit afraid on how much work will be to support negating the conditions, but seems that the input didn't had any weird edge cases and only had to worry about two operations. 

## Day 20

Part 1 was pretty easy, although I had some struggle with the borrow checker because I was storing `&str` while iterating on the System. For part 2 I don't have code since I just relied on the structure to find the minimum presses for each of the inputs of the *Conjunction* module connected to the output.

### Day 21

Part 1 could be simulated easily. For part 2 the simulation gets expensive pretty fast, but I was able to run around the couple of thousands steps, so I could analize the data in a spreadsheet. By playing with data I managed to find the "linear" relationship on the data over the period, so I tested with the first couple of inputs until I managed to get a formula to reliable reproduce the rest. 

### Day 22

Today I had made quickly a first version that didn't work properly, I tried to find the issue for a while and even checked Reddit for extra inputs to see if maybe my algorithm wasn't correct. In the end my issue was that I was so sure of my collision that I didn't review it much and it wasn't until I made the "slow" version that I realized that the mistake was there. Luckily over the long debug I optimized the algorithm to a version that made part 2 much easier.

### Day 23

Today second part I thought I could also brute force it since I didn't think the slopes would decrease the problem space so much. I had to write a new algorithm to simplify the maze into a graph of junctions with distances and there it was possible to find the longest route.

### Day 24 

I did the first part using vector math that I know relatively well, but I left second part until I was forced by day 25 to finish the 49 stars, and then I took the easy route of plugging the equations into Z3. I tried to get the bindings working in my laptop, but since it seemed to not work straight out of the box, I just printed the SMT commands and manually run `z3 day24.smt2`

### Day 25 

This year has been the hardest of the 3 I've done the event, I was suprised by the complexity of "Christmas day" problem, so I just decided to take the easy route after playing around with a couple of algorithms that didn't work and relied on a graph algorithm crate `rustworkx` to get the min cut using [Stoer–Wagner algorithm](https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm).
