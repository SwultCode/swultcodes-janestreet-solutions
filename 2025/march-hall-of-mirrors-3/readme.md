# Hall of Mirrors 3 (March 2025)

This puzzle involves a grid-based problem with mirrors and beams. The solution uses:

- Depth-first search (DFS) for finding valid configurations

**Solution Approach:**
- Implemented a board representation with nodes, mirrors, and grid cells
- Used a factor-based movement system where nodes can only move distances that are factors of their value
- Applied DFS to explore possible configurations
- Optimized with a global lookup table for number factors
- Final solution: 601931086080


## Example Output:

```
Initial board state:
  -    -    -   112   -    48 3087    9   -    -     1   -  
  -  [   ][   ][   ][   ][   ][   ][   ][   ][   ][   ]  -  
  -  [   ][   ][   ][   ][   ][   ][   ][   ][   ][   ]   4 
  -  [   ][   ][   ][   ][   ][   ][   ][   ][   ][   ]  27 
  27 [   ][   ][   ][   ][   ][   ][   ][   ][   ][   ]  -  
  -  [   ][   ][   ][   ][   ][   ][   ][   ][   ][   ]  -  
  -  [   ][   ][   ][   ][   ][   ][   ][   ][   ][   ]  -  
  -  [   ][   ][   ][   ][   ][   ][   ][   ][   ][   ]  16 
  12 [   ][   ][   ][   ][   ][   ][   ][   ][   ][   ]  -  
 225 [   ][   ][   ][   ][   ][   ][   ][   ][   ][   ]  -  
  -  [   ][   ][   ][   ][   ][   ][   ][   ][   ][   ]  -  
  -  2025   -    -    12   64    5   -   405   -    -    -  

Solution found at depth 56
Solution found in 6.69ms:
  -    -    -    -    -    -    -    -              -    -  
                  \                   \              \      
                                                \        -  
  -     /                   /              /             -  
  -               /                                  \      
                                      /                     
  -          /              \              \                
        /         \              /              /        -  
                       \                                 -  
  -                         /         /         /        -  
  -     /                        /                          
  -    -         -         -    -         -    -         -  

Final solution: 601931086080

Process finished with exit code 0
```