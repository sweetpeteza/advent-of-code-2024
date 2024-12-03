from itertools import tee

import logging


def process(input_str: str) -> str:
    reports = []

    # Parse the input lines into a list of integers
    for line in input_str.splitlines():
        levels = list(map(int, line.split()))
        reports.append(levels)

    # Function to check if a sequence is strictly increasing or decreasing
    def is_strictly_increasing_or_decreasing(seq):
        # Check if the list is strictly increasing or strictly decreasing
        return all(curr < next for curr, next in zip(seq, seq[1:])) or all(
            curr > next for curr, next in zip(seq, seq[1:])
        )

    # Function to check if all the differences between adjacent numbers are less than 4
    def has_small_differences(seq):
        return all(abs(curr - next) < 4 for curr, next in zip(seq, seq[1:]))

    # Count how many sequences are safe based on the conditions
    total_safe_tests = 0
    for report in reports:
        # Check if the sequence has no duplicates and meets other conditions
        if len(report) == len(set(report)):  # No duplicates
            if is_strictly_increasing_or_decreasing(report) and has_small_differences(
                report
            ):
                total_safe_tests += 1

    return str(total_safe_tests)


# Test function
def test_process():
    input_str = """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""

    assert process(input_str) == "2"


test_process()

# Configure logging (equivalent to tracing_subscriber::fmt::init() in Rust)
logging.basicConfig(level=logging.INFO)


def main():
    try:
        # Open the input file (equivalent to include_str!("../../input1.txt") in Rust)
        with open("../../rust/day-02/input1.txt", "r") as file:
            input_data = file.read()

        # Call the process function and print the result
        result = process(input_data)
        print(result)
    except Exception as e:
        logging.error(f"Error processing part 1: {e}")
        raise


if __name__ == "__main__":
    main()
