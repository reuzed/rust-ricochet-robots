# Performance of Solver

For the first solver run, commit `7f33d0625f69667b391c6983e1fd55c7b4cdab01`:
Move number 0. Visited: 0, Current: 1, Solved: 0, Time: 192ns
Move number 1. Visited: 1, Current: 18, Solved: 0, Time: 362.911µs
Move number 2. Visited: 19, Current: 302, Solved: 0, Time: 8.679522ms
Move number 3. Visited: 309, Current: 4695, Solved: 0, Time: 136.452877ms
Move number 4. Visited: 4008, Current: 66302, Solved: 0, Time: 2.036379722s
Move number 5. Visited: 40024, Current: 827065, Solved: 0, Time: 28.101799273s

## After seeding rng and preallocate

Move number 0. Visited: 0, Current: 1, Solved: 0, Time: 177ns, Log current: 0
Move number 1. Visited: 1, Current: 18, Solved: 0, Time: 327.876µs, Log current: 0.9648298
Move number 2. Visited: 19, Current: 299, Solved: 0, Time: 7.233141ms, Log current: 1.9028548
Move number 3. Visited: 304, Current: 4617, Solved: 0, Time: 120.718348ms, Log current: 2.8165066
Move number 4. Visited: 3888, Current: 64568, Solved: 0, Time: 1.822829875s, Log current: 3.697084
Move number 5. Visited: 38412, Current: 796791, Solved: 0, Time: 26.837089923s, Log current: 4.5359015

Move number 0. Visited: 0, Current: 1, Solved: 0, Time: 202ns, Log current: 0
Move number 1. Visited: 1, Current: 18, Solved: 0, Time: 397.515µs, Log current: 0.9648298
Move number 2. Visited: 19, Current: 299, Solved: 0, Time: 7.926348ms, Log current: 1.9028548
Move number 3. Visited: 304, Current: 4617, Solved: 0, Time: 134.804486ms, Log current: 2.8165066
Move number 4. Visited: 3888, Current: 64568, Solved: 0, Time: 1.754839735s, Log current: 3.697084
Move number 5. Visited: 38412, Current: 796791, Solved: 0, Time: 24.307849681s, Log current: 4.5359015

## Don't clone grid (5x speedup)

Instead of cloning Grid, use `Arc<Grid>` to share reference.
Move number 0. Visited: 0, Current: 1, Solved: 0, Time: 194ns, Log current: 0
Move number 1. Visited: 1, Current: 18, Solved: 0, Time: 99.417µs, Log current: 0.9648298
Move number 2. Visited: 19, Current: 299, Solved: 0, Time: 2.108195ms, Log current: 1.9028548
Move number 3. Visited: 304, Current: 4617, Solved: 0, Time: 33.223139ms, Log current: 2.8165066
Move number 4. Visited: 3888, Current: 64568, Solved: 0, Time: 423.231252ms, Log current: 3.697084
Move number 5. Visited: 38412, Current: 796791, Solved: 0, Time: 5.971249215s, Log current: 4.5359015
Move number 6. Visited: 282787, Current: 8453116, Solved: 0, Time: 75.154204164s, Log current: 5.324256
