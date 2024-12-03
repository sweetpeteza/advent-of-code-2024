import logging
from typing import List, Tuple
import time


def process(input: str) -> str:
    left = []
    right = []

    for line in input.splitlines():
        items = line.split()
        left.append(int(items[0]))
        right.append(int(items[1]))

    left.sort()
    right.sort()

    total_distance = sum(abs(l - r) for l, r in zip(left, right))

    return str(total_distance)


# Test the function
def test_process():
    input_data = """3   4
4   3
2   5
1   3
3   9
3   3"""
    result = process(input_data)
    assert result == "11", f"Expected '11' but got {result}"


# Run the test
test_process()

# Configure logging (equivalent to tracing_subscriber::fmt::init() in Rust)
logging.basicConfig(level=logging.INFO)


def main():
    try:
        # Open the input file (equivalent to include_str!("../../input1.txt") in Rust)
        with open("../../rust/day-01/input1.txt", "r") as file:
            input_data = file.read()

        # Call the process function and print the result
        start_time = time.perf_counter()
        result = process(input_data)
        end_time = time.perf_counter()  # End the timer

        elapsed_time = end_time - start_time
        print(f"The function took {elapsed_time:.8f} seconds.")
        print(result)
    except Exception as e:
        logging.error(f"Error processing part 1: {e}")
        raise


if __name__ == "__main__":
    main()
