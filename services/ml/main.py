import pyspark as ps 

spark = ps.sql.SparkSession.builder.getOrCreate()


#returns a dataframe
def data()->ps.sql.DataFrame:
    data = [{"name": "Alice", "age": 10,"likes": "apples"},]
    return spark.createDataFrame(data)

#q:what is the best type of machine learning model to use for probabilistic predictions?
#a: a random forest model

model = spark.randomForestClassifier()

print(data().show())
