# Sorting Practice
implementing sorting function from "Algorithm Design Manual" book by Steven S. Skiena into rust

## Benchmark
The result is the average time taken by the sort function in 10 iterations
$n$ is the number of elements in the array

| Sort Function | $n = 100$ | $n = 10,000$ | $n = 1,000,000$ |
| -- | -- | -- | -- |
| Heapsort (slow construct) | 108.55µs | 22.19ms | 3.44s |
| Heapsort (fast construct, mutate original array) |  62.77µs | 11.99ms | 1.99s |
| Mergesort | 101.99µs | 12.05ms | 1.53s |
| Quicksort (mutate original array) | 46.5µs | 6.67ms | 866.62ms |