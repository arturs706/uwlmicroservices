# Use the official Node.js 16.x image as the base image
FROM node:20 AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the package.json and package-lock.json (or pnpm-lock.yaml) to the container
COPY package.json pnpm-lock.yaml ./

# Install `pnpm` globally to run the app's build
RUN npm install -g pnpm

# Install the project dependencies using `pnpm`
RUN pnpm install

# Copy the rest of the app source code to the container
COPY . .

# Build the Solid.js app using Vite
RUN pnpm run build

# Use a lightweight Nginx image to serve the built app
FROM nginx:alpine

# Remove the default Nginx configuration
RUN rm /etc/nginx/conf.d/default.conf

# Copy your custom Nginx configuration file to the container
COPY nginx.conf /etc/nginx/conf.d

# Copy the built app from the builder stage to the Nginx web root directory
COPY --from=builder /app/dist /usr/share/nginx/html

# Expose port 80 for the Nginx server
EXPOSE 80

# Start the Nginx server when the container runs
CMD ["nginx", "-g", "daemon off;"]
