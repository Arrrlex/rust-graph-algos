from hodgerank import hodgerank
import numpy as np
import pytest

prefs_list = [
    {
        "prefs": {
            0: np.array([5, 3]),
            1: np.array([4, 5]),
            2: np.array([3, np.nan]),
            3: np.array([5, 2]),
        },
        "expected": {
            0: 0.4642857142857137,
            1: 0.7499999999999991,
            2: -1.2499999999999987,
            3: 0.0357142857142852,
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
            0: 0.5000000000000002,
            1: 0.9999999999999998,
            2: -1.4999999999999993,
            3: 4.163336342344337e-16,
        },
    },
    {
        "prefs": {
            0: np.array([4, np.nan, 3]),
            1: np.array([3, 4, np.nan]),
            2: np.array([np.nan, 3, 4]),
        },
        "expected": {0: 0.0, 1: 1.3877787807814457e-17, 2: 0.0},
    },
    {
        "prefs": {
            0: np.array([5, np.nan, 3]),
            1: np.array([3, 4, np.nan]),
            2: np.array([np.nan, 3, 4]),
        },
        "expected": {
            0: 0.3333333333333334,
            1: -0.3333333333333334,
            2: -1.1102230246251565e-16,
        },
    },
    {
        "prefs": {
            0: np.array([5, np.nan, 3]),
            1: np.array([3, 4, np.nan]),
            2: np.array([1, 3, 4]),
        },
        "expected": {
            0: 1.0000000000000002,
            1: 5.551115123125783e-17,
            2: -1.0000000000000004,
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
            0: 0.6357142857142856,
            1: 1.1357142857142855,
            2: -0.971428571428572,
            3: 0.3142857142857142,
            4: -1.114285714285714,
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
            "a": 0.4642857142857137,
            "b": 0.7499999999999991,
            "c": -1.2499999999999987,
            "d": 0.0357142857142852,
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
