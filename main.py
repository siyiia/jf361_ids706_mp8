import pandas as pd
import psutil
import time


def analyse_data(file_path):
    # Load the data
    df = pd.read_csv(file_path)

    means = df.mean()
    std_devs = df.std()

    stats_data = pd.DataFrame({"mean": means, "std": std_devs}).T
    return df.columns.tolist(), stats_data


def display_stats(headers, stats_data):
    print(f"{'':<10}", end="")
    for header in headers:
        print(f"{header:<15}", end="")
    print()

    for label, row in stats_data.iterrows():
        print(f"{label:<10}", end="")
        for value in row:
            print(f"{value:<15.2f}", end="")
        print()


def main():
    start_time = time.time()

    file_path = "StudentPerformanceFactors.csv"

    process = psutil.Process()
    initial_memory = process.memory_info().rss / 1024

    headers, stats_data = analyse_data(file_path)

    elapsed_time = time.time() - start_time
    final_memory = process.memory_info().rss / 1024
    memory_used = final_memory - initial_memory

    display_stats(headers, stats_data)

    print(f"\nTime taken: {elapsed_time:.4f} seconds")
    print(f"Memory used: {memory_used:.2f} KB")


if __name__ == "__main__":
    main()
