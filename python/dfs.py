from typing import Hashable, TypeVar
import pytest

A = TypeVar("A", bound=Hashable)
B = TypeVar("B", bound=Hashable)


def dfs(network: list[tuple[A, B]]):
    """Compute connected components using depth-first search."""
    # Build adjacency list.
    adj = {}
    for a, b in network:
        adj.setdefault(a, set()).add(b)
        adj.setdefault(b, set()).add(a)

    # Run DFS.
    visited = set()
    ccs = []
    for node in adj:
        if node not in visited:
            cc = set()
            stack = [node]
            while stack:
                node = stack.pop()
                if node not in visited:
                    visited.add(node)
                    cc.add(node)
                    stack.extend(adj[node])
            ccs.append(cc)

    a_nodes = {a for a, _ in network}
    return [{n for n in cc if n in a_nodes} for cc in ccs]


@pytest.mark.parametrize("f", [dfs])
def test_cc(f):
    data = [
        ("A", "12"),
        ("B", "12"),
        ("B", "13"),
        ("C", "13"),
        ("C", "1045"),
        ("D", "1045"),
        ("D", "2095"),
        ("E", "2095"),
        ("E", "883"),
        ("E", "6634"),
        ("F", "6634"),
        ("F", "7777"),
        ("G", "7777"),
        ("M", "7768"),
        ("N", "7768"),
        ("N", "8998"),
        ("O", "7768"),
        ("O", "9000"),
        ("P", "9000"),
    ]

    ccs = [
        {"A", "B", "C", "D", "E", "F", "G"},
        {"M", "N", "O", "P"},
    ]

    assert f(data) == ccs
