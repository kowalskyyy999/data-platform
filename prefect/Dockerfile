# Use an official Python image
FROM python:3.10

# Set the working directory
WORKDIR /app

ENV PREFECT_API_DATABASE_CONNECTION_URL=""
# Install Prefect
RUN pip install --no-cache-dir prefect

RUN prefect config set PREFECT_API_DATABASE_CONNECTION_URL=$PREFECT_API_DATABASE_CONNECTION_URL

# Expose the Prefect server port (default: 4200)
EXPOSE 4200

# Default command
CMD ["prefect", "server", "start", "--host", "0.0.0.0"]

