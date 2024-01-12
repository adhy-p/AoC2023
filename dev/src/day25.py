import networkx as nx
import matplotlib.pyplot as plt

def main():
    G = nx.Graph()
    with open("input.example.1") as f:
        file = f.readlines()
        for line in file:
            source, destination = line.split(': ')
            for d in destination.strip().split(' '):
                G.add_edge(source, d)

    nx.draw(G)
    nx.drawing.nx_pydot.write_dot(G,'graph.dot')
    _, (p1, p2) = nx.stoer_wagner(G)
    print(len(p1) * len(p2))


if __name__ == "__main__":
    main()