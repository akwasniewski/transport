import osmnx as ox
import networkx as nx

place = "Kraków, Polska"

G = ox.graph_from_place(place, network_type="drive", simplify=True)

print(f"Liczba węzłów: {len(G.nodes)}")
print(f"Liczba krawędzi: {len(G.edges)}")

G_undirected = G.to_undirected()

snap_file = "graphs/krakow_snap.txt"
nx.write_edgelist(G_undirected, snap_file, data=False)

print(f"Network exported to {snap_file} in SNAP format.")