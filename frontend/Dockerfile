# Use an official Node.js runtime as a parent image with a specific version
FROM node:20-alpine

# Set the working directory in the container
WORKDIR /app

# Install app dependencies by copying package.json and package-lock.json first
COPY package*.json ./

RUN npm install

# Copy the rest of the application code
COPY . .

# Build the app for production
RUN npm run build

# Expose port 3000 for the server
EXPOSE 3000

# Use serve to run the app
RUN npm install -g serve
CMD ["serve", "-s", "build"]

