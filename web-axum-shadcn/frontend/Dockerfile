# syntax=docker/dockerfile:1

# ---- Build the static SPA ---------------------------------------------
FROM node:24-alpine AS build
WORKDIR /app
RUN npm install --global --force yarn@1.22.22

COPY package.json yarn.lock .npmrc ./
RUN yarn install --frozen-lockfile

COPY . .
# VITE_API_BASE_URL stays empty: nginx serves the app and the API on one origin.
RUN yarn build

# ---- Serve with nginx --------------------------------------------------
FROM nginx:stable-alpine
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/build /usr/share/nginx/html
EXPOSE 80
