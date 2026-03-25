# Devlog
## Week 1
### 13 march
* set up a simple python file to export the graph data from osmnx (vibe coded)
* set up rust parser (vibe coded)
* set up graph structure
* immplemented dijkstra
* implemented astar -> it gives slightly wrong results as probably the distance function needs to be in 3d, will fix it later

### 14 march
* fixed astar (i read edge file as coordinates xdddd)
* tried and failed to wire up my old graph visualisation tool which used wgpu -> github.com/akwasniewski/clique-oxide/ (too much has changed in wgpu)
* instead vibecoded a visualisation tool which uses eframe, works nice

### 16-17 march
* wired up the visualisation tool to algorithms
* manual refactor 

## Week 2
* implemented bidirectional dijsktra
* refactor 

## Week 3
* implemented astar with two heuristsis the, h_b=-h_f one and the average one 
* made claude change a visualisation tool so I can highlight vertices and choose an algorithm
