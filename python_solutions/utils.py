import time
import functools

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