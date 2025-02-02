import os
from pyspark.sql import SparkSession
from pyspark.sql.functions import from_json, col
from pyspark.sql.types import StructType, StructField, DecimalType, StringType, LongType

def main():

    # Create a Spark session with Kafka and Iceberg dependencies
    spark = SparkSession.builder \
        .appName("KafkaToIceberg") \
        .config("spark.jars.packages", "org.apache.spark:spark-sql-kafka-0-10_2.12:3.4.0,org.apache.iceberg:iceberg-spark-runtime-3.4_2.12:1.4.0") \
        .config("spark.sql.catalog.demo1", "org.apache.iceberg.spark.SparkCatalog") \
        .config("spark.sql.catalog.demo1.catalog-impl", "org.apache.iceberg.rest.RESTCatalog") \
        .config("spark.sql.catalog.demo1.uri", os.getenv("REST_URI")) \
        .config("spark.sql.catalog.demo1.io-impl", "org.apache.iceberg.aws.s3.S3FileIO") \
        .config("spark.sql.catalog.demo1.warehouse", os.getenv("S3_WAREHOUSE")) \
        .config("spark.sql.catalog.demo1.s3.endpoint", os.getenv("S3_ENDPOINT")) \
        .config("spark.sql.catalog.demo1.s3.use-arn-region-enabled", "false") \
        .getOrCreate()
    
    # Read data from Kafka
    kafka_stream = spark.readStream \
    .format("kafka") \
    .option("kafka.bootstrap.servers", "kafka-server:9092") \
    .option("subscribe", "binance_usdt_topic") \
    .option("startingOffsets", "earliest") \
    .load()

    message_schema = StructType([
    StructField("c", DecimalType(20, 16), True),
    StructField("p", DecimalType(20, 16), True),
    StructField("s", StringType(), True),
    StructField("t", LongType(), True),
    StructField("v", DecimalType(20, 16), True)])

    try:
        spark.sql("""
CREATE TABLE demo1.finnhub_db.binance_usdt_tab (
    c DECIMAL(20, 16),
    p DECIMAL(20, 16),
    s STRING,
    t BIGINT,
    v DECIMAL(20, 16)
) USING iceberg;
""")
    except:
        print("Table has already created")

    # Parse the Kafka message using the schema
    df = kafka_stream.selectExpr("CAST(value AS STRING) as json_value") \
    .withColumn("data", from_json(col("json_value"), message_schema)) \
    .select(
        col("data.c").alias("c"),
        col("data.p").alias("p"),
        col("data.s").alias("s"),
        col("data.t").alias("t"),
        col("data.v").alias("v")
    )

    # Write the parsed data into the Iceberg table
    query = df.writeStream.format("iceberg").outputMode("append").option("path", "demo1.finnhub_db.binance_usdt_tab").option("checkpointLocation", "/tmp/spark-checkpoints/binance_usdt_table").start()
    # Wait for the query to terminate
    query.awaitTermination()

if __name__ == "__main__":
    main()
