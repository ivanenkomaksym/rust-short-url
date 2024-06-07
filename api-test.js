import http from "k6/http";
import { check, sleep } from 'k6';

export const options = {
  stages: [
    { duration: "10s", target: 20 },
  ]
};

const randomLinks = [
  "https://example.com/link1",
  "https://example.com/link2",
  "https://example.com/link3",
  "https://example.com/link4",
  "https://example.com/link5",
  "https://example.com/link6",
  "https://example.com/link7",
  "https://example.com/link8",
  "https://example.com/link9",
  "https://example.com/link10"
];

export async function setup() {
  const shortenedLinks = [];

  for (let i = 0; i < randomLinks.length; i++) {
    const randomLink = randomLinks[i];
    const response = http.get(`http://localhost/shorten?long_url=${encodeURIComponent(randomLink)}`);
    if (response.status == 200) {
      const shortenedLink = response.body;
      shortenedLinks.push(shortenedLink);
    } else {
      console.error(`Failed to shorten link: ${randomLink}`, response.error);
    }
  }

  return { shortenedLinks: shortenedLinks };
}

export default function () {
  const response = http.get('http://localhost/urls');

  check(response, { 'response code was 200': (res) => res.status == 200 });

  sleep(1);
}

export async function teardown(data) {
  for (let i = 0; i < data.shortenedLinks.length; i++) {
    const shortenedLink = data.shortenedLinks[i];
    console.log(`shortenedLink: ${shortenedLink}`);
    const response = http.del(`http://${shortenedLink}`);
    if (response.status != 204) {
      console.error(`Failed to remove link: ${shortenedLink}`, response.code);
    }
  }
}