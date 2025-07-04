import createClient from "openapi-fetch";
import type { paths } from "./paths";

const client = createClient<paths>({ baseUrl: "http://localhost:3001/api/" });
export default client;
