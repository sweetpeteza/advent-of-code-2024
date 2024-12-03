from collections import defaultdict
from typing import List
import logging


def process(input: str) -> str:
    left = []
    right = []

    for line in input.splitlines():
        items = line.split()
        left.append(int(items[0]))
        right.append(int(items[1]))

    # Use defaultdict to emulate HashMap behavior
    right_totals = defaultdict(int)

    for num in right:
        right_totals[num] += 1

    similarity_score = sum(x * right_totals.get(x, 0) for x in left)

    print(similarity_score)  # Equivalent to Rust's dbg!
    return str(similarity_score)


# Test the function
def test_process():
    input_data = """3   4
4   3
2   5
1   3
3   9
3   3"""
    result = process(input_data)
    assert result == "31", f"Expected '31' but got {result}"


# Run the test
test_process()

# Configure logging (equivalent to tracing_subscriber::fmt::init() in Rust)
logging.basicConfig(level=logging.INFO)


def main():
    try:
        # Open the input file (equivalent to include_str!("../../input1.txt") in Rust)
        with open("../../rust/day-01/input2.txt", "r") as file:
            input_data = file.read()

        # Call the process function and print the result
        result = process(input_data)
        print(result)
    except Exception as e:
        logging.error(f"Error processing part 2: {e}")
        raise


if __name__ == "__main__":
    main()
