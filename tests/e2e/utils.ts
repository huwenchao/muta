import fetch from "node-fetch";
import { createHttpLink } from "apollo-link-http";
import { InMemoryCache } from "apollo-cache-inmemory";
import ApolloClient from "apollo-client";

export const API_URL = "http://localhost:8000/graphql";
export const client = new ApolloClient({
  link: createHttpLink({
    uri: API_URL,
    fetch: fetch
  }),
  cache: new InMemoryCache(),
  defaultOptions: { query: { fetchPolicy: "no-cache" } }
});

export function makeid(length: number) {
  var result = "";
  var characters = "abcdef0123456789";
  var charactersLength = characters.length;
  for (var i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

export function getNonce() {
  return makeid(64);
}

export function delay(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
