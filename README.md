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

Production builds use `https://chartify-client.onrender.com` as the backend origin, configured in `.env.production`. To use a different backend, set or override `VITE_API_BASE_URL` in the Netlify site settings and trigger a new deploy. The backend must be publicly reachable over HTTPS and allow requests from the deployed Netlify site through CORS.
