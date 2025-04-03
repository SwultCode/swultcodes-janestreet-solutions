# Hall of Mirrors 3 (March 2025)

This puzzle involves a grid-based problem with mirrors and beams. The solution uses:

- Depth-first search (DFS) for finding valid configurations

**Solution Approach:**
- Implemented a board representation with nodes, mirrors, and grid cells
- Used a factor-based movement system where nodes can only move distances that are factors of their value
- Applied DFS to explore possible configurations
- Optimized with a global lookup table for number factors
- Final solution: 601931086080