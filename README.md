# Chartify

Chartify turns text into an editorial information board with visualized metrics, comparisons, and concise takeaways. This repository contains the Vue 3 + Vite frontend; the backend is maintained separately.

## Local development

```bash
cd frontend
npm ci
npm run dev
```

The development server runs at `http://localhost:5173`. By default, the frontend sends analysis requests to `http://localhost:3001`. To use another backend, set `VITE_API_BASE_URL` to its origin, without an `/api` path suffix.

## Netlify deployment

The root `netlify.toml` configures Netlify to build from `frontend/` using `npm run build` and publish `dist/`. The `public/_redirects` file enables client-side routes.

In the Netlify site settings, add the environment variable `VITE_API_BASE_URL` with the public origin of the deployed backend, then trigger a new deploy. The backend must be publicly reachable over HTTPS and allow requests from the deployed Netlify site through CORS. Without this variable, the frontend builds and loads, but analysis requests are unavailable in production.
