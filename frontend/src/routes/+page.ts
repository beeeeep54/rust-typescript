import client from "$lib/api/index.js";

export async function load({ fetch }) {
  try {
    const hello = await client.GET("/hello", {
      fetch,
    });
    return hello.data;
  } catch (error) {
    // Handle CORS or header-related errors gracefully
    console.error("API request failed:", error);

    // You could return default data or throw a more user-friendly error
    return {
      error: "Failed to load data",
      data: null,
    };
  }
}
