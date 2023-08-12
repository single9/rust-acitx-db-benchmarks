# Benchmarks

## Environment

**Hardware**

- CPU: Intel 13th i7-13700KF
- RAM: 128 GB
- SSD: Kingston KC3000 PCIe 4.0 NVMe M.2 SSD 1TB

**Software**

- Ubuntu 22.04
- Docker 20.10.21
- Rust 1.65.0
- PostgreSQL 14 Alpine (Official Image)

## Hello World

### actix-web

```
  execution: local
     script: benchmark/hello_world.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 10m30s max duration (incl. graceful stop):
           * default: 100000 iterations shared among 1000 VUs (maxDuration: 10m0s, gracefulStop: 30s)


running (00m00.5s), 0000/1000 VUs, 100000 complete and 0 interrupted iterations
default ✓ [======================================] 1000 VUs  00m00.5s/10m0s  100000/100000 shared iters

     data_received..................: 8.7 MB 17 MB/s
     data_sent......................: 13 MB  25 MB/s
     http_req_blocked...............: avg=186.76µs min=203ns   med=827ns  max=94.32ms p(90)=2.01µs  p(95)=2.59µs
     http_req_connecting............: avg=180.62µs min=0s      med=0s     max=94.3ms  p(90)=0s      p(95)=0s
     http_req_duration..............: avg=2.55ms   min=20.42µs med=1.65ms max=92.6ms  p(90)=5.32ms  p(95)=7.73ms
       { expected_response:true }...: avg=2.55ms   min=20.42µs med=1.65ms max=92.6ms  p(90)=5.32ms  p(95)=7.73ms
     http_req_failed................: 0.00%  ✓ 0             ✗ 100000
     http_req_receiving.............: avg=67.7µs   min=2.45µs  med=8.24µs max=39.68ms p(90)=15.14µs p(95)=32.54µs
     http_req_sending...............: avg=42.39µs  min=1.2µs   med=3.71µs max=37.73ms p(90)=7.71µs  p(95)=24.7µs
     http_req_tls_handshaking.......: avg=0s       min=0s      med=0s     max=0s      p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=2.44ms   min=13.58µs med=1.61ms max=92.59ms p(90)=5.17ms  p(95)=7.19ms
     http_reqs......................: 100000 191135.070929/s
     iteration_duration.............: avg=3.77ms   min=32.36µs med=2.34ms max=99.8ms  p(90)=7.62ms  p(95)=12.59ms
     iterations.....................: 100000 191135.070929/s
```

## Create Todo

    k6 run --vus 1000 --iterations 100000 benchmark/create_todo.js

### actix-web + diesel

```
  execution: local
     script: benchmark/create_todo.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 10m30s max duration (incl. graceful stop):
           * default: 100000 iterations shared among 1000 VUs (maxDuration: 10m0s, gracefulStop: 30s)


     data_received..................: 28 MB  4.1 MB/s
     data_sent......................: 16 MB  2.4 MB/s
     http_req_blocked...............: avg=247.63µs min=215ns  med=706ns   max=108.21ms p(90)=1.15µs   p(95)=1.3µs
     http_req_connecting............: avg=243.93µs min=0s     med=0s      max=108ms    p(90)=0s       p(95)=0s
     http_req_duration..............: avg=66.68ms  min=4.97ms med=43.51ms max=666.03ms p(90)=139.15ms p(95)=184.79ms
       { expected_response:true }...: avg=66.68ms  min=4.97ms med=43.51ms max=666.03ms p(90)=139.15ms p(95)=184.79ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 100000
     http_req_receiving.............: avg=12.73µs  min=4.21µs med=12µs    max=456.34µs p(90)=17.03µs  p(95)=20.8µs
     http_req_sending...............: avg=10.48µs  min=1.56µs med=4.41µs  max=103.35ms p(90)=6.93µs   p(95)=7.59µs
     http_req_tls_handshaking.......: avg=0s       min=0s     med=0s      max=0s       p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=66.65ms  min=4.96ms med=43.49ms max=666.01ms p(90)=139.13ms p(95)=184.75ms
     http_reqs......................: 100000 14691.373811/s
     iteration_duration.............: avg=66.95ms  min=5ms    med=43.71ms max=666.06ms p(90)=139.54ms p(95)=185.53ms
     iterations.....................: 100000 14691.373811/s
     vus............................: 1000   min=1000       max=1000
     vus_max........................: 1000   min=1000       max=1000
```

### actix-web + sqlx

```
  execution: local
     script: benchmark/create_todo.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 10m30s max duration (incl. graceful stop):
           * default: 100000 iterations shared among 1000 VUs (maxDuration: 10m0s, gracefulStop: 30s)


running (00m06.8s), 0000/1000 VUs, 100000 complete and 0 interrupted iterations
default ✓ [======================================] 1000 VUs  00m06.8s/10m0s  100000/100000 shared iters

     data_received..................: 28 MB  4.0 MB/s
     data_sent......................: 16 MB  2.4 MB/s
     http_req_blocked...............: avg=206.83µs min=211ns  med=658ns   max=107.41ms p(90)=1µs     p(95)=1.12µs
     http_req_connecting............: avg=199.75µs min=0s     med=0s      max=81.12ms  p(90)=0s      p(95)=0s
     http_req_duration..............: avg=67.12ms  min=5.85ms med=65.93ms max=100.21ms p(90)=72.16ms p(95)=77.08ms
       { expected_response:true }...: avg=67.12ms  min=5.85ms med=65.93ms max=100.21ms p(90)=72.16ms p(95)=77.08ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 100000
     http_req_receiving.............: avg=10.4µs   min=3.82µs med=9.69µs  max=1.61ms   p(90)=15.02µs p(95)=16.36µs
     http_req_sending...............: avg=22.75µs  min=1.5µs  med=3.91µs  max=54.03ms  p(90)=6.04µs  p(95)=6.63µs
     http_req_tls_handshaking.......: avg=0s       min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=67.08ms  min=5.33ms med=65.91ms max=100.15ms p(90)=72.09ms p(95)=77.04ms
     http_reqs......................: 100000 14612.197674/s
     iteration_duration.............: avg=67.38ms  min=9.14ms med=65.96ms max=171.2ms  p(90)=72.73ms p(95)=78.07ms
     iterations.....................: 100000 14612.197674/s
     vus............................: 1000   min=1000       max=1000
     vus_max........................: 1000   min=1000       max=1000
```

## Get Todo List

    k6 run --vus 1000 --iterations 100000 benchmark/get_todo_list.js

### actix-web + diesel

```
  execution: local
     script: benchmark/get_todo_list.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 10m30s max duration (incl. graceful stop):
           * default: 100000 iterations shared among 1000 VUs (maxDuration: 10m0s, gracefulStop: 30s)


     data_received..................: 1.7 GB 749 MB/s
     data_sent......................: 14 MB  6.1 MB/s
     http_req_blocked...............: avg=354.58µs min=202ns  med=741ns   max=105.98ms p(90)=1.07µs  p(95)=1.34µs
     http_req_connecting............: avg=350.62µs min=0s     med=0s      max=104.62ms p(90)=0s      p(95)=0s
     http_req_duration..............: avg=21.42ms  min=1.82ms med=12.31ms max=424.36ms p(90)=51.57ms p(95)=72.77ms
       { expected_response:true }...: avg=21.42ms  min=1.82ms med=12.31ms max=424.36ms p(90)=51.57ms p(95)=72.77ms
     http_req_failed................: 0.00%  ✓ 0           ✗ 100000
     http_req_receiving.............: avg=18.73µs  min=5.74µs med=15.51µs max=10.15ms  p(90)=22.37µs p(95)=25.98µs
     http_req_sending...............: avg=14.99µs  min=1.25µs med=3.97µs  max=10.81ms  p(90)=5.65µs  p(95)=6.57µs
     http_req_tls_handshaking.......: avg=0s       min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=21.39ms  min=1.8ms  med=12.28ms max=423.87ms p(90)=51.49ms p(95)=72.65ms
     http_reqs......................: 100000 44279.83024/s
     iteration_duration.............: avg=21.81ms  min=1.84ms med=12.33ms max=460.99ms p(90)=52.23ms p(95)=74.87ms
     iterations.....................: 100000 44279.83024/s
     vus............................: 1000   min=1000      max=1000
     vus_max........................: 1000   min=1000      max=1000
```

### actix-web + sqlx

```
  execution: local
     script: benchmark/get_todo_list.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 10m30s max duration (incl. graceful stop):
           * default: 100000 iterations shared among 1000 VUs (maxDuration: 10m0s, gracefulStop: 30s)


running (00m07.0s), 0000/1000 VUs, 100000 complete and 0 interrupted iterations
default ✓ [======================================] 1000 VUs  00m07.0s/10m0s  100000/100000 shared iters

     data_received..................: 1.7 GB 243 MB/s
     data_sent......................: 14 MB  2.0 MB/s
     http_req_blocked...............: avg=127.95µs min=202ns  med=669ns   max=64.99ms  p(90)=1.02µs  p(95)=1.14µs
     http_req_connecting............: avg=118.84µs min=0s     med=0s      max=44.65ms  p(90)=0s      p(95)=0s
     http_req_duration..............: avg=68.63ms  min=2.49ms med=67.47ms max=102.59ms p(90)=77.17ms p(95)=78.98ms
       { expected_response:true }...: avg=68.63ms  min=2.49ms med=67.47ms max=102.59ms p(90)=77.17ms p(95)=78.98ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 100000
     http_req_receiving.............: avg=15.19µs  min=6.17µs med=14.05µs max=1.29ms   p(90)=21.5µs  p(95)=23.93µs
     http_req_sending...............: avg=14.77µs  min=1.34µs med=3.39µs  max=38.89ms  p(90)=5.13µs  p(95)=5.64µs
     http_req_tls_handshaking.......: avg=0s       min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=68.6ms   min=2ms    med=67.45ms max=87.26ms  p(90)=77.14ms p(95)=78.96ms
     http_reqs......................: 100000 14377.372891/s
     iteration_duration.............: avg=68.79ms  min=4.37ms med=67.5ms  max=115.25ms p(90)=77.19ms p(95)=79.01ms
     iterations.....................: 100000 14377.372891/s
     vus............................: 1000   min=1000       max=1000
     vus_max........................: 1000   min=1000       max=1000
```

## Get A Todo Item

    k6 run --vus 1000 --iterations 100000 benchmark/get_todo.js

### actix-web + diesel

```
  execution: local
     script: benchmark/get_todo.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 10m30s max duration (incl. graceful stop):
           * default: 100000 iterations shared among 1000 VUs (maxDuration: 10m0s, gracefulStop: 30s)


     data_received..................: 28 MB  17 MB/s
     data_sent......................: 13 MB  7.8 MB/s
     http_req_blocked...............: avg=145.4µs  min=206ns    med=723ns   max=104.76ms p(90)=1.05µs  p(95)=1.29µs
     http_req_connecting............: avg=137.11µs min=0s       med=0s      max=76.64ms  p(90)=0s      p(95)=0s
     http_req_duration..............: avg=15.43ms  min=472.98µs med=8.92ms  max=272.95ms p(90)=35.76ms p(95)=50.3ms
       { expected_response:true }...: avg=15.43ms  min=472.98µs med=8.92ms  max=272.95ms p(90)=35.76ms p(95)=50.3ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 100000
     http_req_receiving.............: avg=11.79µs  min=3.25µs   med=10.31µs max=26.73ms  p(90)=15.04µs p(95)=16.32µs
     http_req_sending...............: avg=37.01µs  min=1.26µs   med=3.76µs  max=43.47ms  p(90)=5.4µs   p(95)=5.96µs
     http_req_tls_handshaking.......: avg=0s       min=0s       med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=15.38ms  min=460.45µs med=8.89ms  max=272.94ms p(90)=35.65ms p(95)=50.23ms
     http_reqs......................: 100000 60807.855085/s
     iteration_duration.............: avg=15.61ms  min=497.58µs med=8.97ms  max=272.97ms p(90)=36.29ms p(95)=51.39ms
     iterations.....................: 100000 60807.855085/s
     vus............................: 1000   min=1000       max=1000
     vus_max........................: 1000   min=1000       max=1000
```

### actix-web + sqlx

```
  execution: local
     script: benchmark/get_todo.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 10m30s max duration (incl. graceful stop):
           * default: 100000 iterations shared among 1000 VUs (maxDuration: 10m0s, gracefulStop: 30s)


running (00m02.3s), 0000/1000 VUs, 100000 complete and 0 interrupted iterations
default ✓ [======================================] 1000 VUs  00m02.3s/10m0s  100000/100000 shared iters

     data_received..................: 28 MB  12 MB/s
     data_sent......................: 13 MB  5.6 MB/s
     http_req_blocked...............: avg=250.88µs min=201ns  med=546ns   max=82.17ms  p(90)=956ns   p(95)=1.08µs
     http_req_connecting............: avg=226.11µs min=0s     med=0s      max=81.45ms  p(90)=0s      p(95)=0s
     http_req_duration..............: avg=22.28ms  min=5.55ms med=20.54ms max=81.08ms  p(90)=28.16ms p(95)=31.05ms
       { expected_response:true }...: avg=22.28ms  min=5.55ms med=20.54ms max=81.08ms  p(90)=28.16ms p(95)=31.05ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 100000
     http_req_receiving.............: avg=9.43µs   min=3.58µs med=8.36µs  max=1.22ms   p(90)=14.18µs p(95)=15.51µs
     http_req_sending...............: avg=6.44µs   min=1.3µs  med=2.84µs  max=26.95ms  p(90)=4.82µs  p(95)=5.3µs
     http_req_tls_handshaking.......: avg=0s       min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=22.26ms  min=5.32ms med=20.52ms max=80.07ms  p(90)=28.14ms p(95)=31.03ms
     http_reqs......................: 100000 43086.393233/s
     iteration_duration.............: avg=22.55ms  min=7.86ms med=20.59ms max=109.24ms p(90)=28.6ms  p(95)=31.51ms
     iterations.....................: 100000 43086.393233/s
     vus............................: 1000   min=1000       max=1000
     vus_max........................: 1000   min=1000       max=1000
```
