
# Dijkstra's Algorithm: A General Overview

Dijkstra's algorithm is a popular algorithm used for finding the shortest paths between nodes in a graph, which may 
represent, for example, road networks. It was conceived by computer scientist Edsger W. Dijkstra in 1956 and published three years later.

## What it Does

At its core, Dijkstra's algorithm finds the shortest path from a single source node to all other nodes in a graph, provided that all edge 
weights are non-negative. Think of it like finding the quickest route from your home to various destinations on a map, where the roads
have different travel times (weights) and there are no shortcuts that take you back in time (negative weights).

The algorithm works by iteratively exploring the most "promising" unvisited node, updating the shortest known distance to its 
neighbors, until all reachable nodes have been visited and the shortest path to each is determined.

## Steps of the Algorithm

Here's a simplified breakdown of how Dijkstra's algorithm generally proceeds:

1.  **Initialization:**
    *   Assign a distance value to every node. The source node gets a distance of 0, and all other nodes get an initial distance 
    of infinity (or a very large number), indicating they are currently unreachable.
    *   Mark all nodes as unvisited.

2.  **Selection of Current Node:**
    *   From the set of unvisited nodes, select the one with the smallest current distance value. This node becomes the "current node."

3.  **Update Neighbors:**
    *   For each neighbor of the current node:
        *   Calculate a new distance to the neighbor by adding the current node's distance to the weight of the edge connecting them.
        *   If this new calculated distance is smaller than the neighbor's current recorded distance, update the neighbor's distance.

4.  **Mark as Visited:**
    *   Once all neighbors of the current node have been processed, mark the current node as visited. It will not be considered again.

5.  **Repeat:**
    *   Continue steps 2-4 until all nodes have been visited or until the smallest distance among unvisited nodes is infinity 
    (meaning the remaining unvisited nodes are unreachable from the source).

By following these steps, the algorithm systematically explores the graph, ensuring that by the time a node is marked as 
visited, the shortest path to it from the source has been found.
