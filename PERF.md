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

## New solver approach

Move number (1) 0. Visited: 1, Current: 0, Solved: 0, Time: 810ns
Move number (2) 1. Visited: 19, Current: 17, Solved: 0, Time: 57.347µs
Move number (3) 19. Visited: 304, Current: 284, Solved: 0, Time: 3.176123ms
Move number (4) 304. Visited: 3888, Current: 3583, Solved: 0, Time: 431.13857ms
Move number (5) 3888. Visited: 38412, Current: 34523, Solved: 0, Time: 47.210800057s

Add hashset: (30x speedup)
Move number (1) 0. Visited: 1, Current: 0, Solved: 0, Time: 806ns
Move number (2) 1. Visited: 19, Current: 17, Solved: 0, Time: 317.994µs
Move number (3) 19. Visited: 304, Current: 284, Solved: 0, Time: 6.886989ms
Move number (4) 304. Visited: 3888, Current: 3583, Solved: 0, Time: 84.409343ms
Move number (5) 3888. Visited: 38412, Current: 34523, Solved: 0, Time: 731.903213ms
Move number (6) 38412. Visited: 282787, Current: 244374, Solved: 0, Time: 5.851275383s
Move number (7) 282787. Visited: 1527926, Current: 1245138, Solved: 0, Time: 36.363213231s
Move number (8) 1527926. Visited: 6354846, Current: 4826919, Solved: 0, Time: 175.264441683s
Move number (9) 6354846. Visited: 21890517, Current: 15535670, Solved: 0, Time: 668.700467715s

## After sad bugfix

Move number (1) 0. Visited: 1, Current: 0, Solved: 0, Time: 760ns
Move number (2) 1. Visited: 19, Current: 17, Solved: 0, Time: 279.732µs
Move number (3) 19. Visited: 302, Current: 282, Solved: 0, Time: 4.84745ms
Move number (4) 302. Visited: 3816, Current: 3513, Solved: 0, Time: 70.630607ms
Move number (5) 3816. Visited: 36958, Current: 33141, Solved: 0, Time: 655.648886ms
Move number (6) 36958. Visited: 264261, Current: 227302, Solved: 0, Time: 5.52336093s
Move number (7) 264261. Visited: 1377314, Current: 1113052, Solved: 0, Time: 32.429624379s

Quicker hash of Position (ignore grid)
Move number (1) 0. Visited: 1, Current: 0, Solved: 0, Time: 720ns
Move number (2) 1. Visited: 19, Current: 17, Solved: 0, Time: 110.511µs
Move number (3) 19. Visited: 302, Current: 282, Solved: 0, Time: 1.721963ms
Move number (4) 302. Visited: 3816, Current: 3513, Solved: 0, Time: 25.188325ms
Move number (5) 3816. Visited: 36958, Current: 33141, Solved: 0, Time: 250.576426ms
Move number (6) 36958. Visited: 264261, Current: 227302, Solved: 0, Time: 2.101826023s
Move number (7) 264261. Visited: 1377314, Current: 1113052, Solved: 0, Time: 13.629264297s
Move number (8) 1377314. Visited: 5558009, Current: 4180694, Solved: 0, Time: 66.170398744s

Also sort robots, kills many states
Move number (1) 0. Visited: 1, Current: 0, Solved: 0, Time: 1.05µs
Move number (2) 1. Visited: 19, Current: 17, Solved: 0, Time: 118.482µs
Move number (3) 19. Visited: 182, Current: 162, Solved: 0, Time: 1.299463ms
Move number (4) 182. Visited: 1162, Current: 979, Solved: 0, Time: 11.172099ms
Move number (5) 1162. Visited: 5721, Current: 4558, Solved: 0, Time: 70.830561ms
Move number (6) 5721. Visited: 23479, Current: 17757, Solved: 0, Time: 305.386723ms
Move number (7) 23479. Visited: 84842, Current: 61362, Solved: 0, Time: 1.128519738s
Move number (8) 84842. Visited: 279655, Current: 194812, Solved: 0, Time: 3.977620748s
Move number (9) 279655. Visited: 858034, Current: 578378, Solved: 0, Time: 11.689974943s

Add Fx hasher:
Move number (1) 0. Visited: 1, Current: 0, Solved: 0, Time: 1.051µs
Move number (2) 1. Visited: 19, Current: 17, Solved: 0, Time: 117.238µs
Move number (3) 19. Visited: 182, Current: 162, Solved: 0, Time: 897.572µs
Move number (4) 182. Visited: 1162, Current: 979, Solved: 0, Time: 6.195412ms
Move number (5) 1162. Visited: 5721, Current: 4558, Solved: 0, Time: 37.022067ms
Move number (6) 5721. Visited: 23479, Current: 17757, Solved: 0, Time: 184.965599ms
Move number (7) 23479. Visited: 84842, Current: 61362, Solved: 0, Time: 676.3889ms
Move number (8) 84842. Visited: 279655, Current: 194812, Solved: 0, Time: 2.443605725s
Move number (9) 279655. Visited: 858034, Current: 578378, Solved: 0, Time: 7.561878499s
Move number (10) 858034. Visited: 2477413, Current: 1619378, Solved: 0, Time: 23.644767162s
Move number (11) 2477413. Visited: 6772484, Current: 4295070, Solved: 0, Time: 67.332708532s
Move number (12) 6772484. Visited: 17575568, Current: 10803083, Solved: 3, Time: 201.634888383s
