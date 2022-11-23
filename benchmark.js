import http from 'k6/http';

export default function () {
  const url = 'http://127.0.0.1:3000/api/todo/';
  const payload = JSON.stringify({
    "title": "title" + Date.now() + Math.round(Math.random() * 1000),
  });
  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };
  http.post(url, payload, params);
}
