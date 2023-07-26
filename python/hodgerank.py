"""
Alternative hodgerank implementation, bypassing networkx.

Based on http://niplav.site/notes.html#Nothing_to_See_Here_Just_An_Implementation_of_HodgeRank
"""
from typing import Collection, Mapping, TypeVar
import warnings
import numpy as np

K = TypeVar("K")

warnings.simplefilter("ignore", category=RuntimeWarning)


def prefgraph(
    prefs: np.ndarray[int, int]
) -> tuple[np.ndarray[int, int], np.ndarray[int, int]]:
    """Collapse voter preferences to single preference graph as an adjacency matrix."""

    prefs_delta = prefs[:, None, :] - prefs[None, :, :]

    counts = np.sum(~np.isnan(prefs_delta), axis=2)

    weights = np.nanmean(prefs_delta, axis=2)
    weights[weights < 0] = 0

    return weights, counts


def decompose(
    weights: np.ndarray[int, int], counts: np.ndarray[int, int]
) -> np.ndarray[float]:
    """Apply the HodgeRank decomposition to obtain a ranking."""
    n_edges = np.count_nonzero(counts)
    n_nodes = counts.shape[0]

    nonzero_indices = counts.flatten().nonzero()[0]
    f = weights.flatten()[nonzero_indices]
    W = np.diag(counts.flatten()[nonzero_indices])

    signs = np.sign(weights).flatten()[nonzero_indices]
    starts, ends = counts.nonzero()

    origins = np.zeros((n_edges, n_nodes))
    origins[np.arange(n_edges), starts] = -signs
    origins[np.arange(n_edges), ends] = signs

    try:
        s = -np.linalg.pinv(origins.T @ W @ origins) @ origins.T @ W @ f
    except np.linalg.LinAlgError:
        s = np.zeros(n_nodes)

    return s


def pref_array(
    prefs: Mapping[K, Collection[int]]
) -> tuple[Mapping[K, int], np.ndarray[int, int]]:
    """Convert a preference mapping to a numpy array."""

    num_voters = len(prefs[list(prefs.keys())[0]])
    num_options = len(prefs)
    idx = {n: i for i, n in enumerate(prefs.keys())}
    _prefs = np.zeros((num_options, num_voters))
    for k, v in prefs.items():
        _prefs[idx[k], :] = list(v)

    return idx, _prefs


def hodgerank(prefs: Mapping[K, np.ndarray[int]]) -> Mapping[K, float]:
    idx, _prefs = pref_array(prefs)
    weights, counts = prefgraph(_prefs)
    hodge_prefs = decompose(weights, counts)
    return {k: hodge_prefs[idx[k]] for k in prefs.keys()}
