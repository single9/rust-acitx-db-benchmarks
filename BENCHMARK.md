# Benchmarks

## Environment

**Hardware**

- CPU: Intel 13th i7-13700KF
- RAM: 128 GB
- SSD: Kingston KC3000 PCIe 4.0 NVMe M.2 SSD 1TB

**Software**

- Ubuntu 22.04
- Docker 20.10.21
- Rust 1.71.1
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


     data_received..................: 28 MB  4.1 MB/s
     data_sent......................: 16 MB  2.4 MB/s
     http_req_blocked...............: avg=238.36µs min=199ns  med=645ns   max=90.61ms  p(90)=1.1µs   p(95)=1.26µs
     http_req_connecting............: avg=223.23µs min=0s     med=0s      max=89.61ms  p(90)=0s      p(95)=0s
     http_req_duration..............: avg=66.38ms  min=3.57ms med=64.93ms max=91.16ms  p(90)=73.71ms p(95)=75.21ms
       { expected_response:true }...: avg=66.38ms  min=3.57ms med=64.93ms max=91.16ms  p(90)=73.71ms p(95)=75.21ms
     http_req_failed................: 0.00%  ✓ 0           ✗ 100000
     http_req_receiving.............: avg=15.88µs  min=3.93µs med=10.86µs max=9.35ms   p(90)=15.4µs  p(95)=17.46µs
     http_req_sending...............: avg=22.06µs  min=1.5µs  med=4.07µs  max=59.38ms  p(90)=6.48µs  p(95)=7.13µs
     http_req_tls_handshaking.......: avg=0s       min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=66.34ms  min=3.22ms med=64.92ms max=87.58ms  p(90)=73.68ms p(95)=75.19ms
     http_reqs......................: 100000 14796.27543/s
     iteration_duration.............: avg=66.65ms  min=9.24ms med=64.97ms max=163.79ms p(90)=73.89ms p(95)=75.36ms
     iterations.....................: 100000 14796.27543/s
     vus............................: 1000   min=1000      max=1000
     vus_max........................: 1000   min=1000      max=1000
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


     data_received..................: 1.7 GB 350 MB/s
     data_sent......................: 14 MB  2.9 MB/s
     http_req_blocked...............: avg=234.89µs min=195ns   med=717ns   max=111.07ms p(90)=1.07µs  p(95)=1.24µs
     http_req_connecting............: avg=229.34µs min=0s      med=0s      max=110.95ms p(90)=0s      p(95)=0s
     http_req_duration..............: avg=47.15ms  min=11.46ms med=45.38ms max=103.4ms  p(90)=55.62ms p(95)=56.6ms
       { expected_response:true }...: avg=47.15ms  min=11.46ms med=45.38ms max=103.4ms  p(90)=55.62ms p(95)=56.6ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 100000
     http_req_receiving.............: avg=15.97µs  min=5.92µs  med=14.84µs max=5.27ms   p(90)=21.34µs p(95)=23.72µs
     http_req_sending...............: avg=29.38µs  min=1.29µs  med=3.76µs  max=86.22ms  p(90)=5.34µs  p(95)=5.88µs
     http_req_tls_handshaking.......: avg=0s       min=0s      med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=47.1ms   min=10.97ms med=45.36ms max=102.94ms p(90)=55.59ms p(95)=56.56ms
     http_reqs......................: 100000 20695.489054/s
     iteration_duration.............: avg=47.41ms  min=11.51ms med=45.41ms max=167.44ms p(90)=55.71ms p(95)=56.81ms
     iterations.....................: 100000 20695.489054/s
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


     data_received..................: 28 MB  13 MB/s
     data_sent......................: 13 MB  5.9 MB/s
     http_req_blocked...............: avg=34.07µs min=188ns    med=554ns   max=52.04ms p(90)=1µs     p(95)=1.15µs
     http_req_connecting............: avg=29.78µs min=0s       med=0s      max=52.01ms p(90)=0s      p(95)=0s
     http_req_duration..............: avg=20.86ms min=110.82µs med=21.06ms max=71.96ms p(90)=22.4ms  p(95)=31.19ms
       { expected_response:true }...: avg=20.86ms min=110.82µs med=21.06ms max=71.96ms p(90)=22.4ms  p(95)=31.19ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 100000
     http_req_receiving.............: avg=10.6µs  min=2.33µs   med=8.71µs  max=15.03ms p(90)=14.04µs p(95)=15.31µs
     http_req_sending...............: avg=19.41µs min=1.05µs   med=2.96µs  max=67.65ms p(90)=5.04µs  p(95)=5.57µs
     http_req_tls_handshaking.......: avg=0s      min=0s       med=0s      max=0s      p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=20.83ms min=106.88µs med=21.04ms max=33.93ms p(90)=22.39ms p(95)=28.86ms
     http_reqs......................: 100000 45921.326934/s
     iteration_duration.............: avg=20.91ms min=119.32µs med=21.08ms max=77.3ms  p(90)=22.43ms p(95)=31.3ms
     iterations.....................: 100000 45921.326934/s
     vus............................: 1000   min=1000       max=1000
     vus_max........................: 1000   min=1000       max=1000
```
