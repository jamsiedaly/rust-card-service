import http from 'k6/http';
import { check } from 'k6';

export let options = {
  stages: [
    { duration: '10s', target: 10 },
    { duration: '20s', target: 100 },
    { duration: '30s', target: 100 },
//    { duration: '30s', target: 100 },
//    { duration: '20s', target: 10 },
    { duration: '20s', target: 0 }
  ],
};

export default function () {
  var url = 'https://rust-card-service.azurewebsites.net/api/card';
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

  var res = http.post(url, payload, params);
  check(res, {
    'response code was 200': (res) => res.status == 200,
    'card number is valid': (res) => res.body == "valid",
  });
}

