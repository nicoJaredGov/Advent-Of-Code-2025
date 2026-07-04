import numpy as np
import os
from utils import timer_wrapper, get_sorted_pairs, get_area_mat

@timer_wrapper
def sol(file_name: str) -> int:
    file_path = os.path.abspath(f'{os.path.dirname(__file__)}/{file_name}.txt')
    with open(file_path, mode="r") as file:
        X = np.array([[int(x) for x in l.split(',')] for l in file.read().splitlines()])

    A = get_area_mat(X)
    p, q = get_sorted_pairs(A)[::-1][0]
    a, b = np.abs(X[p] - X[q])

    return (a + 1) * (b + 1)


sol("../inputs/example/day9")
sol("../inputs/day9")