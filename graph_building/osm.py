import osmnx as ox
import networkx as nx

place = "Polska"

# Download road network
G = ox.graph_from_place(place, network_type="drive", simplify=True)

# Remove isolated nodes
isolated_nodes = list(nx.isolates(G))
G.remove_nodes_from(isolated_nodes)

print(f"Liczba węzłów: {len(G.nodes)}")
print(f"Liczba krawędzi: {len(G.edges)}")

# Convert node labels to integers
G_normalized = nx.convert_node_labels_to_integers(G, first_label=0, ordering='default')

# Export edges with distances instead of travel times
snap_file = "graphs/polska_snap.txt"
with open(snap_file, 'w') as f:
    for u, v, data in G_normalized.edges(data=True):
        length = data.get('length', 0)  # distance in meters
        f.write(f"{u} {v} {length}\n")

# Export node coordinates
coords_file = "graphs/polska_coords.txt"
with open(coords_file, 'w') as f:
    for node, data in G_normalized.nodes(data=True):
        lon = data.get('x', 0)
        lat = data.get('y', 0)
        f.write(f"{node} {lat} {lon}\n")

print(f"Network exported to {snap_file} (edges) and {coords_file} (node coordinates).")
