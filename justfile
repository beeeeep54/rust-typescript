set dotenv-load

dev:
    zellij --layout dev-layout.kdl

backend-dev:
    cd backend && watchexec -i 'target/**' cargo run

frontend-dev:
    cd frontend && bun run dev --port "${FRONTEND_PORT}"
