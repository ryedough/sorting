# Sorting Practice
implementing sorting into rust from "Algorithm Design Manual" book by Steven S. Skiena

## Benchmark
The result were average from 10 runs of each sort function
$n$ is the number of elements in the array

| Sort Function | $n = 100$ | $n = 10,000$ | $n = 1,000,000$ |
| -- | -- | -- | -- |
| Heapsort (slow construct) | 108.55µs | 22.19ms | 3.44s |
| Heapsort (fast construct) |  94.58µs | 20.88ms | 3.12s |
| Mergesort | 101.99µs | 12.05ms | 1.53s |
| Quicksort (mutate original array) | 46.5µs | 6.67ms | 866.62ms |