from python import Python
from python import logging


def process(input: str) -> str:
    # Import Python's `list` and `zip` functionalities
    py_list = Python.builtins.list
    py_zip = Python.builtins.zip
    py_abs = Python.builtins.abs
    py_int = Python.builtins.int

    left = py_list()
    right = py_list()

    # Split lines and process input
    for line in input.splitlines():
        items = line.split()
        left.append(py_int(items[0]))
        right.append(py_int(items[1]))

    # Sort the lists
    left.sort()
    right.sort()

    # Calculate the total distance
    total_distance = sum(py_abs(l - r) for l, r in py_zip(left, right))
    return str(total_distance)


def test_process():
    input_data = """3   4
4   3
2   5
1   3
3   9
3   3"""
    result = process(input_data)
    assert result == "11", f"Expected '11' but got {result}"

# Main function


def main():
    logging.basicConfig(level=logging.INFO)
    log = logging.getLogger()

    try:
        # Import Python's file handling
        open_file = Python.builtins.open

        # Open and read input file
        with open_file("../../rust/day-01/input1.txt", "r") as file:
            input_data = file.read()

        # Process the input data
        result = process(input_data)
        print(result)
    except Exception as e:
        log.error(f"Error processing part 1: {e}")
        raise


if __name__ == "__main__":
    main()
