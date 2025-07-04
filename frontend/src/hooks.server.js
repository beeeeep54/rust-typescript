/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }) {
  return await resolve(event, {
    filterSerializedResponseHeaders(name) {
      return ["content-type", "content-length"].includes(name.toLowerCase());
    },
  });
}
