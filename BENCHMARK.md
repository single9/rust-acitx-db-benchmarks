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

## Create Todo

    k6 run --vus 1000 --iterations 100000 benchmark/create_todo.js

### web-actix + diesel

```
  execution: local
     script: benchmark/create_todo.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 10m30s max duration (incl. graceful stop):
           * default: 100000 iterations shared among 1000 VUs (maxDuration: 10m0s, gracefulStop: 30s)


running (00m06.8s), 0000/1000 VUs, 100000 complete and 0 interrupted iterations
default ✓ [======================================] 1000 VUs  00m06.8s/10m0s  100000/100000 shared iters

     data_received..................: 28 MB  4.1 MB/s
     data_sent......................: 16 MB  2.4 MB/s
     http_req_blocked...............: avg=143.12µs min=211ns  med=757ns   max=93.08ms  p(90)=1.09µs  p(95)=1.21µs
     http_req_connecting............: avg=139.22µs min=0s     med=0s      max=93.06ms  p(90)=0s      p(95)=0s
     http_req_duration..............: avg=66.71ms  min=1.26ms med=66.15ms max=160.59ms p(90)=71.49ms p(95)=76.48ms
       { expected_response:true }...: avg=66.71ms  min=1.26ms med=66.15ms max=160.59ms p(90)=71.49ms p(95)=76.48ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 100000
     http_req_receiving.............: avg=11.73µs  min=3.65µs med=11.24µs max=8.93ms   p(90)=16.2µs  p(95)=17.61µs
     http_req_sending...............: avg=39.42µs  min=1.54µs med=4.51µs  max=52.65ms  p(90)=6.51µs  p(95)=7.12µs
     http_req_tls_handshaking.......: avg=0s       min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=66.66ms  min=1.24ms med=66.13ms max=160.55ms p(90)=71.46ms p(95)=76.43ms
     http_reqs......................: 100000 14693.648978/s
     iteration_duration.............: avg=66.89ms  min=1.28ms med=66.18ms max=160.88ms p(90)=71.55ms p(95)=76.59ms
     iterations.....................: 100000 14693.648978/s
     vus............................: 1000   min=1000       max=1000
     vus_max........................: 1000   min=1000       max=1000
```

### web-actix + sqlx

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

### web-actix + diesel

```
  execution: local
     script: benchmark/get_todo_list.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 10m30s max duration (incl. graceful stop):
           * default: 100000 iterations shared among 1000 VUs (maxDuration: 10m0s, gracefulStop: 30s)


running (00m02.8s), 0000/1000 VUs, 100000 complete and 0 interrupted iterations
default ✓ [======================================] 1000 VUs  00m02.8s/10m0s  100000/100000 shared iters

     data_received..................: 1.7 GB 606 MB/s
     data_sent......................: 14 MB  4.9 MB/s
     http_req_blocked...............: avg=345.28µs min=195ns   med=568ns   max=129.32ms p(90)=1.02µs  p(95)=1.27µs
     http_req_connecting............: avg=337.92µs min=0s      med=0s      max=129.29ms p(90)=0s      p(95)=0s
     http_req_duration..............: avg=26.56ms  min=10.44ms med=25.05ms max=131.07ms p(90)=33.5ms  p(95)=37.14ms
       { expected_response:true }...: avg=26.56ms  min=10.44ms med=25.05ms max=131.07ms p(90)=33.5ms  p(95)=37.14ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 100000
     http_req_receiving.............: avg=32.95µs  min=5.99µs  med=13.06µs max=14.63ms  p(90)=22.82µs p(95)=31.69µs
     http_req_sending...............: avg=16.21µs  min=1.27µs  med=3.06µs  max=63.13ms  p(90)=5.36µs  p(95)=6.92µs
     http_req_tls_handshaking.......: avg=0s       min=0s      med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=26.51ms  min=9.31ms  med=25.02ms max=129.73ms p(90)=33.43ms p(95)=37.03ms
     http_reqs......................: 100000 35813.532072/s
     iteration_duration.............: avg=26.93ms  min=10.51ms med=25.12ms max=162.4ms  p(90)=33.94ms p(95)=37.72ms
     iterations.....................: 100000 35813.532072/s
     vus............................: 1000   min=1000       max=1000
     vus_max........................: 1000   min=1000       max=1000
```

### web-actix + sqlx

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

### web-actix + diesel

```
  execution: local
     script: benchmark/get_todo.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 10m30s max duration (incl. graceful stop):
           * default: 100000 iterations shared among 1000 VUs (maxDuration: 10m0s, gracefulStop: 30s)


running (00m01.7s), 0000/1000 VUs, 100000 complete and 0 interrupted iterations
default ✓ [======================================] 1000 VUs  00m01.7s/10m0s  100000/100000 shared iters

     data_received..................: 28 MB  16 MB/s
     data_sent......................: 13 MB  7.5 MB/s
     http_req_blocked...............: avg=362.19µs min=191ns    med=534ns   max=115.8ms  p(90)=993ns   p(95)=1.23µs
     http_req_connecting............: avg=355.17µs min=0s       med=0s      max=115.47ms p(90)=0s      p(95)=0s
     http_req_duration..............: avg=16.03ms  min=159.2µs  med=14.81ms max=71.37ms  p(90)=20.71ms p(95)=23.11ms
       { expected_response:true }...: avg=16.03ms  min=159.2µs  med=14.81ms max=71.37ms  p(90)=20.71ms p(95)=23.11ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 100000
     http_req_receiving.............: avg=23.29µs  min=3.73µs   med=8.13µs  max=18.79ms  p(90)=14.17µs p(95)=17.23µs
     http_req_sending...............: avg=25.61µs  min=1.24µs   med=2.82µs  max=14.8ms   p(90)=4.95µs  p(95)=6.27µs
     http_req_tls_handshaking.......: avg=0s       min=0s       med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=15.98ms  min=141.19µs med=14.8ms  max=71.33ms  p(90)=20.61ms p(95)=22.95ms
     http_reqs......................: 100000 58031.300941/s
     iteration_duration.............: avg=16.43ms  min=191.52µs med=14.84ms max=136.9ms  p(90)=20.89ms p(95)=23.85ms
     iterations.....................: 100000 58031.300941/s
     vus............................: 1000   min=1000       max=1000
     vus_max........................: 1000   min=1000       max=1000
```

### web-actix + sqlx

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
