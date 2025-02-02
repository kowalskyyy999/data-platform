import subprocess
from prefect import flow, task
from prefect_shell import ShellOperation

@task
def define_is_append(is_append: bool):
    if is_append:
        return "/usr/local/bin/sheet-connector --append"
    return "/usr/local/bin/sheet-connector"

@flow
def run(sheet_id: str, table_name: str, is_append: bool):
    script = define_is_append(is_append)
    script = f"{script} --sheet-id {sheet_id} --table-name {table_name}"
    return subprocess.Popen(script, shell=True)

if __name__ == "__main__":
    import os

    sheet_id = str(os.getenv("SHEET_ID"))
    table_name = str(os.getenv("TABLE_NAME"))
    is_append = True if str(os.getenv("APPEND")).lower() == 'true' else False
    run.serve(parameters={"sheet_id": sheet_id, "table_name": table_name, "is_append": is_append}, cron = "* * * * *")
