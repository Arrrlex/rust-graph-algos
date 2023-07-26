from hodgerank import hodgerank
import numpy as np
import pytest

prefs_list = [
    {
        "prefs": {
            0: [5, 3],
            1: [4, 5],
            2: [3, np.nan],
            3: [5, 2],
        },
        "expected": {
            0: 0.464285,
            1: 0.749999,
            2: -1.24999,
            3: 0.035714,
        },
    },
    {
        "prefs": {
            0: np.array([5, 3]),
            1: np.array([4, 5]),
            2: np.array([3, 1]),
            3: np.array([5, 2]),
        },
        "expected": {
            0: 0.500000,
            1: 0.9999999,
            2: -1.4999999,
            3: 4.1633363e-16,
        },
    },
    {
        "prefs": {
            0: np.array([4, np.nan, 3]),
            1: np.array([3, 4, np.nan]),
            2: np.array([np.nan, 3, 4]),
        },
        "expected": {0: 0.0, 1: 1.3877787e-17, 2: 0.0},
    },
    {
        "prefs": {
            0: np.array([5, np.nan, 3]),
            1: np.array([3, 4, np.nan]),
            2: np.array([np.nan, 3, 4]),
        },
        "expected": {
            0: 0.3333333,
            1: -0.3333333,
            2: -1.1102230e-16,
        },
    },
    {
        "prefs": {
            0: np.array([5, np.nan, 3]),
            1: np.array([3, 4, np.nan]),
            2: np.array([1, 3, 4]),
        },
        "expected": {
            0: 1.0000000,
            1: 5.5511151e-17,
            2: -1.0000000,
        },
    },
    {
        "prefs": {0: np.array([1, 1]), 1: np.array([1, 1]), 2: np.array([1, 1])},
        "expected": {0: 0.0, 1: 0.0, 2: 0.0},
    },
    {
        "prefs": {
            0: np.array([5, 3]),
            1: np.array([4, 5]),
            2: np.array([3, np.nan]),
            3: np.array([5, 2]),
            4: np.array([np.nan, 2]),
        },
        "expected": {
            0: 0.6357142,
            1: 1.1357142,
            2: -0.9714285,
            3: 0.3142857,
            4: -1.1142857,
        },
    },
    {
        "prefs": {
            "a": np.array([5, 3]),
            "b": np.array([4, 5]),
            "c": np.array([3, np.nan]),
            "d": np.array([5, 2]),
        },
        "expected": {
            "a": 0.4642857,
            "b": 0.7499999,
            "c": -1.2499999,
            "d": 0.0357142,
        },
    },
]


@pytest.mark.parametrize("prefs", prefs_list)
def test_hodgerank(prefs):
    assert is_close_dict(hodgerank(prefs["prefs"]), prefs["expected"])


def is_close_dict(dict1, dict2):
    for key in set(dict1.keys()) | set(dict2.keys()):
        if not np.isclose(dict1[key], dict2[key]):
            return False
    return True
