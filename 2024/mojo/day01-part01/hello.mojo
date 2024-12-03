fn main():
    print("Hello, world!")

    # Example usage:
    for i in range(10):
        print(fibonacci_recursive(i), end=" ")


fn fibonacci_recursive(n: Int32) -> Int32:
    if n <= 0:
        return 0
    elif n == 1:
        return 1
    else:
        return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)

