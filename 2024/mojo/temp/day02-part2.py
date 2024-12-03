from enum import Enum
import logging


class Direction(Enum):
    Increasing = "increasing"
    Decreasing = "decreasing"


def parse(input_str):
    """
    Parses the input into a list of reports where each report is a list of integers.
    """
    return [list(map(int, line.split())) for line in input_str.strip().splitlines()]


def check_safety(report):
    """
    Checks the safety of the report, which involves checking for specific conditions.
    Returns an exception message if any condition fails.
    """
    direction = None

    for a, b in zip(report, report[1:]):
        diff = a - b
        sign = diff // abs(diff) if diff != 0 else 0  # Signum equivalent

        if sign == -1:
            if direction == Direction.Increasing:
                return f"{a};{b} now increasing"
            elif direction == Direction.Decreasing:
                if not (1 <= abs(diff) <= 3):
                    return f"{a};{b} diff of {abs(diff)}"
                else:
                    continue
            elif direction is None:
                if not (1 <= abs(diff) <= 3):
                    return f"{a};{b} diff of {abs(diff)}"
                else:
                    direction = Direction.Decreasing
                    continue
        elif sign == 1:
            if direction == Direction.Increasing:
                if not (1 <= diff <= 3):
                    return f"{a};{b} diff of {abs(diff)}"
                else:
                    continue
            elif direction == Direction.Decreasing:
                return f"{a};{b} now decreasing"
            elif direction is None:
                if not (1 <= diff <= 3):
                    return f"{a};{b} diff of {abs(diff)}"
                else:
                    direction = Direction.Increasing
                    continue
        elif sign == 0:
            return f"{a};{b} has no diff"
        else:
            raise RuntimeError("Unexpected condition encountered")

    return None


def process(input_str):
    """
    Processes the input string, filters out reports that don't meet safety conditions,
    and counts how many are safe.
    """
    reports = parse(input_str)

    total_safe_tests = 0
    for report in reports:
        error = check_safety(report)
        if error is None:
            total_safe_tests += 1
        else:
            for index in range(len(report)):
                new_report = report[:index] + report[index + 1:]
                error = check_safety(new_report)
                if error is None:
                    total_safe_tests += 1
                    break

    return str(total_safe_tests)


# Test function
def test_process():
    input_str = """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""

    assert process(input_str) == "4"


test_process()

# Configure logging (equivalent to tracing_subscriber::fmt::init() in Rust)
logging.basicConfig(level=logging.INFO)


def main():
    try:
        # Open the input file (equivalent to include_str!("../../input1.txt") in Rust)
        with open("../../rust/day-02/input2.txt", "r") as file:
            input_data = file.read()

        # Call the process function and print the result
        result = process(input_data)
        print(result)
    except Exception as e:
        logging.error(f"Error processing part 2: {e}")
        raise


if __name__ == "__main__":
    main()
