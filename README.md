# Rust + TypeScript Web Application

A web app combining the performance and safety of Rust with TypeScript

## Architecture
- **Backend**: Rust with Poem web framework for API endpoints
- **Frontend**: TypeScript with SvelteKit for interactive user interfaces
- **Build System**: Vite for fast development and optimized production builds
- **Type Safety**: Poem generates an OpenAPI spec which is turned into a type safe client on the frontend

## Development

Start the development servers:

```bash
zellij --config dev-layout.kdl
```

The frontend and backend ports are configured in the env.

## Project Structure

```
├── backend/               
│   ├── build.rs
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── openapi
│   │   └── schema.yaml
│   └── src
│       └── main.rs
├── frontend/               
│   ├── bun.lock
│   ├── package.json
│   ├── src
│   ├── static
│   ├── svelte.config.js
│   ├── tsconfig.json
│   └── vite.config.ts
```
