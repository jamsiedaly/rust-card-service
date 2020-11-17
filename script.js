import http from 'k6/http';

export let options = {
  stages: [
    { duration: '30s', target: 100 }
  ],
};

export default function () {
  var url = 'http://localhost:8080/api/card';
  var payload = JSON.stringify({
    number: '4640207097262595',
    ccv: '123',
    first_name: "John",
    last_name: "Smith"
  });
  var params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };
  http.post(url, payload, params);
}

