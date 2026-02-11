# Ricochet Robots

Ricochet robots is a game consisting of a grid of squares, robots and walls.
Robots may move orthogonally, continnuing in a cardinal direction until they hit a wall.
Robots also collide with other robots.

## Board representation

The board consists of a grid size (m x n)
A set of walls, horizontal or vertical on a specified coordinate (always on top left of their squares (towards 0,0) )
A set of robot positions as coordinates on grid for each robot colour.
A target robot position.

(Cosmetic board decorations like central gray squares, symbols for possible targets)

## Solving algorithm

DFS

## Advanced solving strategies

- Cull board positions that have been reached before by caching
- Explore from final position and meet in the middle
- Use an A^\* type heuristic to guide moves

All these need to be tested for time taken and number of board states visited.
