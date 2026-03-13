import osmnx as ox
import networkx as nx

place = "Kraków, Polska"

G = ox.graph_from_place(place, network_type="drive", simplify=True)
G = ox.add_edge_speeds(G)
G = ox.add_edge_travel_times(G)
isolated_nodes = list(nx.isolates(G))
G.remove_nodes_from(isolated_nodes)

print(f"Liczba węzłów: {len(G.nodes)}")
print(f"Liczba krawędzi: {len(G.edges)}")

G_normalized = nx.convert_node_labels_to_integers(G, first_label=0, ordering='default')

snap_file = "graph_building/krakow_snap.txt"
with open(snap_file, 'w') as f:
    for u, v, data in G_normalized.edges(data=True):
        travel_time = data.get('travel_time', 0)
        f.write(f"{u} {v} {travel_time}\n")


print(f"Network exported to {snap_file} in SNAP format.")