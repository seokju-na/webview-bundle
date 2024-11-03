# Benchmarks

Tested with Mac mini (M1, 2020).

## @webview-bundle/electron

```
✓ index.bench.ts > startup 11229ms
    name                        hz     min     max    mean     p75     p99    p995    p999     rme  samples
  · startup/fs              2.8543  346.72  355.51  350.35  353.80  355.51  355.51  355.51  ±0.71%       10   fastest
  · startup/webview-bundle  2.8198  340.43  373.08  354.63  363.85  373.08  373.08  373.08  ±2.02%       10
✓ index.bench.ts > navigation 18482ms
    name                           hz     min     max    mean     p75     p99    p995    p999     rme  samples
  · navigation/fs              1.6337  599.51  619.99  612.10  616.25  619.99  619.99  619.99  ±0.78%       10   fastest
  · navigation/webview-bundle  1.6210  593.69  630.17  616.92  621.04  630.17  630.17  630.17  ±1.09%       10


BENCH  Summary
 
startup/fs - index.bench.ts > startup
   1.01x faster than startup/webview-bundle
 
navigation/fs - index.bench.ts > navigation
   1.01x faster than navigation/webview-bundle
```

## @webview-bundle/node-binding

```
✓ index.bench.ts > create - next 5232ms
    name                               hz      min      max     mean      p75      p99     p995     p999     rme  samples
  · create - next/zip              113.95   7.4305  12.2770   8.7757  10.2207  11.5936  12.2770  12.2770  ±3.05%      100
  · create - next/zip (compress)  28.9396  32.6592  38.6351  34.5547  35.8997  37.4366  38.6351  38.6351  ±0.83%      100   slowest
  · create - next/webview-bundle   330.48   2.5575   6.3555   3.0259   2.9327   5.9506   6.3555   6.3555  ±3.77%      166   fastest
✓ index.bench.ts > create - react 2001ms
    name                                 hz     min     max    mean     p75     p99    p995    p999     rme  samples
  · create - react/zip               660.03  1.3098  4.3304  1.5151  1.3983  3.7441  3.9039  4.3304  ±3.49%      332
  · create - react/zip (compress)    145.64  6.5390  9.7184  6.8664  6.7956  8.7847  9.7184  9.7184  ±1.52%      100   slowest
  · create - react/webview-bundle  1,052.30  0.8555  4.3875  0.9503  0.9237  3.4459  3.8455  4.3875  ±2.82%      527   fastest


BENCH  Summary

create - next/webview-bundle - index.bench.ts > create - next
  2.90x faster than create - next/zip
  11.42x faster than create - next/zip (compress)

create - react/webview-bundle - index.bench.ts > create - react
  1.59x faster than create - react/zip
  7.23x faster than create - react/zip (compress)
```
