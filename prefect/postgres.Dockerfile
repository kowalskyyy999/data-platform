FROM postgres:15.3

# Install PostgreSQL contrib package (includes pg_term)
RUN apt-get update && apt-get install -y postgresql-contrib

# Ensure PostgreSQL user has permission to create extensions
USER postgres

# Initialize database and enable the extension
RUN mkdir -p /docker-entrypoint-initdb.d
COPY init.sql /docker-entrypoint-initdb.d/
