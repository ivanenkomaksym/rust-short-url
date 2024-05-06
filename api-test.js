import http from "k6/http";
import { check, sleep } from 'k6';

export const options = {
  stages: [
    { duration: "10s", target: 20 },
  ]
};

export default function () {
  const response = http.get('http://localhost/urls');
  
  check(response, { 'response code was 200': (res) => res.status == 200});
  
  sleep(1);
}