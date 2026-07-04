# Deploying gruffed sites

gruffed has two deployable Next.js apps:

- Product site: `packages/site`
- Docs site: `packages/docs`

Use two Vercel projects connected to the same GitHub repository:
`drew-walker/gruffed`.

## Product site

- Project name: `gruffed`
- Root Directory: `packages/site`
- Framework Preset: Next.js
- Install Command: default, or `pnpm install --frozen-lockfile`
- Build Command: default, or `pnpm build`
- Output Directory: default
- Production branch: `main`

Environment variables:

```sh
NEXT_PUBLIC_GRUFFED_DOCS_URL=https://docs.gruffed.dev/docs/cli
```

Use the deployed docs URL instead until a custom docs domain exists.

## Docs site

- Project name: `gruffed-docs`
- Root Directory: `packages/docs`
- Framework Preset: Next.js
- Install Command: default, or `pnpm install --frozen-lockfile`
- Build Command: default, or `pnpm build`
- Output Directory: default
- Production branch: `main`

## Domains

Recommended domain layout:

- `gruffed.dev` -> product site
- `docs.gruffed.dev` -> docs site

The generated Vercel preview domains are fine for alpha testing before buying
or connecting a custom domain.

## Verification

Before deploying from a branch, run:

```sh
pnpm typecheck
pnpm build:sites
```

The GitHub CI workflow already runs the same site checks on `main` and pull
requests.
