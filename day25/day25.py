import networkx as nx

def main():
    G = nx.Graph()
    with open("input") as f:
        file = f.readlines()
        for line in file:
            source, destination = line.split(': ')
            for d in destination.strip().split(' '):
                G.add_edge(source, d)

    _, (p1, p2) = nx.stoer_wagner(G)
    print(len(p1) * len(p2))


if __name__ == "__main__":
    main()