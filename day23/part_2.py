import sys
import networkx as nx


def solution(input: str) -> str:
    G = nx.Graph()

    for line in input.splitlines():
        (left, right) = line.split("-")

        G.add_edge(left, right)

    max_clique = []

    for clique in nx.algorithms.clique.find_cliques(G):
        if len(clique) > len(max_clique):
            max_clique = clique

    return ",".join(sorted(max_clique))


if __name__ == "__main__":
    input_file = sys.argv[1] + ".txt"

    with open(input_file, "r") as fd:
        input = fd.read()

    res = solution(input)
    print(res)
