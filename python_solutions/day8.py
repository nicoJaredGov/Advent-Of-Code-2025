import numpy as np
import os
from utils import timer_wrapper

def get_distance_mat(X: np.ndarray[any]):
    '''
        Given a 2-d matrix of points, calculates the distance matrix for each pair of points.

        Parameters
        ----------
        X : array-like
            (n x d) matrix representing n points in d dimensions
    '''

    # Compute Gram matrix
    G = np.dot(X, X.T)

    # Get squared norms (diagonal of G)
    # Reshape to (n, 1) to enable matrix broadcasting
    p = np.diag(G).reshape(-1, 1)

    # Combine using broadcasting and take square root
    # p + p.T automatically creates the (n x n) matrix addition grid
    D = p + p.T - 2 * G

    return D

def get_sorted_pairs(D: np.ndarray[any]):
    # 1. Get the coordinate arrays (creates index arrays, no matrix data is copied)
    r, c = np.triu_indices_from(D, k=1)

    # 2. Get sorting permutation based on values (creates a small temporary array of just the upper triangle)
    sort_order = np.argsort(D[r, c])

    # 3. Rearrange the indices, NOT the matrix data
    sorted_r = r[sort_order]
    sorted_c = c[sort_order]

    # View the result as pairs
    sorted_pairs = list(zip(sorted_r, sorted_c))

    return sorted_pairs

def build_circuits(circuits, positions, sorted_pairs, evaluate_terminating):
    id = 0

    for (conn1, conn2) in sorted_pairs:
        first = positions.get(conn1)
        second = positions.get(conn2)

        if first is None and second is None:
            circuits[id] = {conn1, conn2}
            positions[conn1] = id
            positions[conn2] = id
            id += 1
        elif first is None:
            positions[conn1] = second
            circuits[second].add(conn1)
        elif second is None:
            positions[conn2] = first
            circuits[first].add(conn2)
        elif first != second:
            for junction in circuits[second]:
                circuits[first].add(junction)
                positions[junction] = first
            del(circuits[second])
        
        if evaluate_terminating():
            return (conn1, conn2)
    
@timer_wrapper
def sol(file_name: str, num_connections: int = 10) -> int:
    file_path = os.path.abspath(f'{os.path.dirname(__file__)}/{file_name}.txt')
    with open(file_path, mode="r") as file:
        X = np.array([[int(x) for x in l.split(',')] for l in file.read().splitlines()])

    D = get_distance_mat(X)
    sorted_pairs = get_sorted_pairs(D)

    circuits: dict[int, set] = dict()
    positions: dict[int, int] = dict()

    def evaluate_terminating():
        nonlocal num_connections
        num_connections -= 1
        return num_connections == 0
    
    build_circuits(circuits, positions, sorted_pairs, evaluate_terminating)
    
    sizes = sorted(map(lambda c: len(c), circuits.values()))
    result = np.prod(sizes[-1:-4:-1])

    return result

@timer_wrapper
def sol2(file_name: str) -> int:
    file_path = os.path.abspath(f'{os.path.dirname(__file__)}/{file_name}.txt')
    with open(file_path, mode="r") as file:
        X = np.array([[int(x) for x in l.split(',')] for l in file.read().splitlines()])

    D = get_distance_mat(X)
    sorted_pairs = get_sorted_pairs(D)

    circuits: dict[int, set] = dict()
    positions: dict[int, int] = dict()

    def evaluate_terminating():
        if len(circuits) != 1:
            return False
        
        for circuit in circuits.values():
            if len(circuit) == len(X):
                return True
            
        return False

    (conn1, conn2) = build_circuits(circuits, positions, sorted_pairs, evaluate_terminating)
    result = X[conn1][0] * X[conn2][0]

    return result
    
sol("../inputs/example/day8", num_connections=10)
sol("../inputs/day8", num_connections=1000)
sol2("../inputs/example/day8")
sol2("../inputs/day8")
