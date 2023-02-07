#trying to predict what the block_type ,chance at a spcific position in a mincraft wolrd is
import random
from pyspark.ml.regression import RandomForestRegressor
from pyspark.sql import SparkSession
from numpy import allclose
from pyspark.ml.linalg import Vector
from dataclasses import dataclass


spark = SparkSession \
    .builder \
    .appName('Prediction Engine') \
    .getOrCreate()

l = [{'name': 'Alice', 'age': 1}, {'name': 'Bob', 'age': 2}]  
df = spark.createDataFrame(l, ['name', 'age'])
print(df.show())

# Split the data into train/test datasets
train_df, test_df = df.randomSplit([.80, .20], seed=42)

# Set hyperparameters for the algorithm
rf = RandomForestRegressor(numTrees=100)

# Fit the model to the training data
model = rf.fit(train_df)

# Generate predictions on the test dataset.
model.transform(test_df).show()
