import time
import functools
import numpy as np

def timer_wrapper(func):
    """A decorator that prints the execution time of a function."""
    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        start_time = time.perf_counter()
        result = func(*args, **kwargs)
        end_time = time.perf_counter()
        
        execution_time = end_time - start_time
        print(f"Executed in {execution_time:.6f} seconds\nResult: {result}\n")
        
        return result
    
    return wrapper

def get_distance_mat(X: np.ndarray[any]):
    '''
        Given a 2-d matrix of points, calculates the distance matrix for each pair of points.
        Distances are relative because square-root is omitted for better performance.

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

def get_area_mat(P: np.ndarray[any]):
    '''
        Given a 2-d matrix of 2-d points, calculates the area matrix.

        Parameters
        ----------
        P : array-like
            (n x d) matrix representing n points in d dimensions
    '''

    X, Y = P[:, [0]], P[:, [1]]

    dx, dy = np.abs(X - X.T), np.abs(Y - Y.T)

    return dx * dy


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