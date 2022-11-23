import http from 'k6/http';

let count = 0;

export default function () {
  const url = 'http://127.0.0.1:3000/api/todo/';
  const payload = JSON.stringify({
    "title": "title_" + count++,
  });
  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };
  http.post(url, payload, params);
}
