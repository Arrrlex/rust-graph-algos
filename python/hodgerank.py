"""
Alternative hodgerank implementation, bypassing networkx.

Based on http://niplav.site/notes.html#Nothing_to_See_Here_Just_An_Implementation_of_HodgeRank
"""

from typing import Collection, Mapping, TypeVar
import warnings
import numpy as np
import numpy.typing as npt

K = TypeVar("K")


def prefgraph(
    prefs: npt.NDArray[np.int_],
) -> tuple[npt.NDArray[np.int_], npt.NDArray[np.int_]]:
    """Collapse voter preferences to single preference graph as an adjacency matrix."""

    prefs_delta = prefs[:, None, :] - prefs[None, :, :]

    counts = np.sum(~np.isnan(prefs_delta), axis=2)

    with warnings.catch_warnings():
        warnings.simplefilter("ignore", category=RuntimeWarning)
        weights = np.nanmean(prefs_delta, axis=2)
    weights[weights < 0] = 0

    return weights, counts


def decompose(
    weights: npt.NDArray[np.int_], counts: npt.NDArray[np.int_]
) -> npt.NDArray[np.float_]:
    """Apply the HodgeRank decomposition to obtain a ranking."""
    n_edges = np.count_nonzero(counts)
    n_nodes = counts.shape[0]

    nonzero_indices = counts.nonzero()
    f = weights[nonzero_indices]
    W = np.diag(counts[nonzero_indices])

    signs = np.sign(weights)[nonzero_indices]

    origins = np.zeros((n_edges, n_nodes))
    origins[np.arange(n_edges), nonzero_indices[0]] = -signs
    origins[np.arange(n_edges), nonzero_indices[1]] = signs

    try:
        s = -np.linalg.pinv(origins.T @ W @ origins) @ origins.T @ W @ f
    except np.linalg.LinAlgError:
        s = np.zeros(n_nodes)

    return s


def convert_prefs_dict_to_array(
    prefs: Mapping[K, Collection[int]]
) -> tuple[Mapping[K, int], npt.NDArray[np.int_]]:
    """Convert a preference mapping to a numpy array."""

    num_voters = len(prefs[list(prefs.keys())[0]])
    num_options = len(prefs)
    idx = {n: i for i, n in enumerate(prefs.keys())}
    _prefs = np.zeros((num_options, num_voters), dtype=np.int_)
    for k, v in prefs.items():
        _prefs[idx[k], :] = list(v)

    return idx, _prefs


def hodgerank(prefs: Mapping[K, npt.NDArray[np.float_]]) -> Mapping[K, float]:
    idx, _prefs = convert_prefs_dict_to_array(prefs)
    weights, counts = prefgraph(_prefs)
    hodge_prefs = decompose(weights, counts)
    return {k: hodge_prefs[idx[k]] for k in prefs.keys()}
