import os 

def main():
    query = f"""ALTER SYSTEM ADD BACKEND "starrocks-be:9050";

ALTER SYSTEM ADD COMPUTE NODE "starrocks-cn:9050";

SET PASSWORD FOR root = PASSWORD('{os.getenv("ROOT_STARROCKS_PASSWORD")}');

CREATE USER {os.getenv('STARROCKS_USERNAME')} IDENTIFIED BY '{os.getenv("STARROCKS_PASSWORD")}' DEFAULT ROLE root;

CREATE DATABASE {os.getenv('STARROCKS_DATABASE')};

CREATE EXTERNAL CATALOG {os.getenv("CATALOG_NAME")}
PROPERTIES
(
    "type" = "iceberg",
    "iceberg.catalog.type" = "rest",
    "iceberg.catalog.uri" = "{os.getenv("CATALOG_REST_URI")}",
    "iceberg.catalog.warehouse" = "{os.getenv("CATALOG_WAREHOUSE")}",
    "aws.s3.enable_ssl" = "false",
    "aws.s3.enable_path_style_access" = "true",
    "aws.s3.endpoint" = "{os.getenv("S3_ENDPOINT")}",
    "aws.s3.access_key" = "{os.getenv("AWS_ACCESS_KEY_ID")}",
    "aws.s3.secret_key" = "{os.getenv("AWS_SECRET_ACCESS_KEY")}"
);
    """

    with open('sql_script.sql', 'w') as f:
        f.write(query)

if __name__ == "__main__":
    main()
