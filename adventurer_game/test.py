import time

N = 10000
ROUNDS = 5000
MOD = 1000


def main():
    start = time.perf_counter()

    total = 0

    for _ in range(ROUNDS):
        for i in range(1, N + 1):
            # Same math as in Rust
            value = (i * i + 3 * i + 7) % MOD
            total = (total + value) % MOD

    end = time.perf_counter()

    print("Result:", total)
    print("Time:", end - start, "seconds")


if __name__ == "__main__":
    main()