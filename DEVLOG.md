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