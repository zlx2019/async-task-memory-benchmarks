benchmark:
	java -jar BenchmarkJava.jar 1000000

build:
	javac BenchmarkJava.java
	echo "Main-Class: BenchmarkJava" > MANIFEST.MF
	jar cvfm BenchmarkJava.jar MANIFEST.MF BenchmarkJava.class