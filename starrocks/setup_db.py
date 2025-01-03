import os 

def main():
    query = f"""ALTER SYSTEM ADD BACKEND "be-server:9050";

ALTER SYSTEM ADD COMPUTE NODE "cn-server:9050";

SET PASSWORD FOR root = PASSWORD('{os.getenv("ROOT_STARROCKS_PASSWORD")}');

CREATE USER {os.getenv('STARROCKS_USERNAME')} IDENTIFIED BY '{os.getenv("STARROCKS_PASSWORD")}' DEFAULT ROLE root;

CREATE DATABASE {os.getenv('STARROCKS_DATABASE')};
    """

    with open('sql_script.sql', 'w') as f:
        f.write(query)

if __name__ == "__main__":
    main()
