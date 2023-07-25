"""From http://niplav.site/notes.html#Nothing_to_See_Here_Just_An_Implementation_of_HodgeRank"""

from typing import Mapping, TypeVar
import numpy as np
import networkx as nx
import itertools as it

K = TypeVar("K")


def positive_edges(prefs):
    for a, b in it.combinations(prefs.keys(), 2):
        pref_delta = prefs[a] - prefs[b]
        if np.all(np.isnan(pref_delta)):
            continue

        weight = np.nanmean(pref_delta)
        n = np.sum(~np.isnan(pref_delta))
        # Add edge in direction where pref_delta is positive
        if weight >= 0:
            yield (a, b, {"weight": weight, "n": n})
        else:
            yield (b, a, {"weight": -weight, "n": n})


def prefgraph(prefs: Mapping[K, np.ndarray[int]]) -> nx.DiGraph:
    """
    Construct a directed graph from a set of preferences from voters.

    prefs is a mapping from options to preference vectors. Each preference vector
    is a numpy array of integers, where the i-th entry is the preference of the
    i-th voter for that option. Higher numbers mean higher preference.
    """
    g = nx.DiGraph()
    g.add_nodes_from(list(prefs.keys()))
    edges = positive_edges(prefs)
    g.add_edges_from(edges)
    return g


def decompose(g):
    f = np.array([g[a][b]["weight"] for a, b in g.edges])
    W = np.diag([g[a][b]["n"] for a, b in g.edges])

    origins = np.zeros((g.number_of_edges(), g.number_of_nodes()))

    idx = {n: i for i, n in enumerate(g.nodes)}

    for c, (a, b) in enumerate(g.edges):
        sign = np.sign(g[a][b]["weight"])
        if np.isnan(sign):
            sign = 0
        origins[c][idx[a]] = -sign
        origins[c][idx[b]] = sign

    try:
        s = -np.linalg.pinv(origins.T @ W @ origins) @ origins.T @ W @ f
    except np.linalg.LinAlgError:
        s = np.zeros(len(list(g.nodes)))

    return {n: s[idx[n]] for n in g.nodes}


def hodgerank(prefs: Mapping[K, np.ndarray[int]]) -> Mapping[K, float]:
    g = prefgraph(prefs)
    return decompose(g)


def main():
    prefs = {
        0: np.array([5, 3]),
        1: np.array([4, 5]),
        2: np.array([3, np.nan]),
        3: np.array([5, 2]),
    }
    print(hodgerank(prefs))


if __name__ == "__main__":
    main()
