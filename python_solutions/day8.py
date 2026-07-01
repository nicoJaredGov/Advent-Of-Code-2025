import numpy as np
import os
import time

def sol(file_name: str, num_connections: int = 10) -> int:
    start_time = time.perf_counter()

    file_path = os.path.abspath(f'{os.path.dirname(__file__)}/{file_name}.txt')
    with open(file_path, mode="r") as file:
        # X is an (n x d) matrix representing n points in d dimensions
        X = np.array([[int(x) for x in l.split(',')] for l in file.read().splitlines()])

        # Step 2: Compute Gram matrix
        G = np.dot(X, X.T)

        # Step 3: Get squared norms (diagonal of G)
        # Reshape to (n, 1) to enable matrix broadcasting
        p = np.diag(G).reshape(-1, 1)

        # Step 4 & 5: Combine using broadcasting and take square root
        # p + p.T automatically creates the (n x n) matrix addition grid
        D = p + p.T - 2 * G

    # 1. Get the coordinate arrays (creates index arrays, no matrix data is copied)
    r, c = np.triu_indices_from(D, k=1)

    # 2. Get sorting permutation based on values (creates a small temporary array of just the upper triangle)
    sort_order = np.argsort(D[r, c])

    # 3. Rearrange the indices, NOT the matrix data
    sorted_r = r[sort_order]
    sorted_c = c[sort_order]

    # View the result as pairs
    sorted_pairs = list(zip(sorted_r, sorted_c))

    circuits: dict[int, set] = dict()
    positions: dict[int, int] = dict()
    id = 0
    for (conn1, conn2) in sorted_pairs:
        found = False

        for (i, circuit) in circuits.items():
            first_found = conn1 in circuit
            second_found = conn2 in circuit

            if first_found and second_found:
                found = True
                break
    
            if first_found or second_found:
                other_junction = conn2 if first_found else conn1
                circuit.add(other_junction)

                # Merge the other junction's circuit with current circuit
                if other_junction in positions:
                    original_pos = positions[other_junction]

                    for junction in circuits[original_pos]:
                        circuit.add(junction)
                        positions[junction] = i
                    
                    del(circuits[original_pos])
                else:
                    positions[other_junction] = i

                found = True
                break

        if not found:
            circuits[id] = {conn1, conn2}
            positions[conn1] = id
            positions[conn2] = id

            id += 1
        
        num_connections -= 1

        if num_connections == 0:
            break

    sizes = sorted(map(lambda c: len(c), circuits.values()))
    result = np.prod(sizes[-1:-4:-1])

    end_time = time.perf_counter()
    elapsed_time = end_time - start_time

    print(f"Execution time: {elapsed_time:.4f} seconds")
    print(result)

    return result

sol("../inputs/example/day8", num_connections=10)
sol("../inputs/day8", num_connections=1000)