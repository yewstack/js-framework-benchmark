# js-framework-benchmark

This is a simple benchmark for several javascript frameworks. The benchmarks creates a large table with randomized entries and measures the time for various operations including rendering duration.

## About the benchmarks

The following operations are benchmarked for each framework:

* create rows: Duration for creating 1,000 rows after the page loaded (no warmup).
* replace all rows: Duration for replacing all 1,000 rows of the table (with 5 warmup iterations).
* partial update: Time to update the text of every 10th row for a table with 10,000 rows (with 5 warmup iterations).
* select row: Duration to highlight a row in response to a click on the row. (with 5 warmup iterations).
* swap rows: Time to swap 2 rows on a table with 1,000 rows. (with 5 warmup iterations).
* remove row: Duration to remove a row for a table with 1,000 rows. (with 5 warmup iterations).
* create many rows: Duration to create 10,000 rows (no warmup)
* append rows to large table: Duration for adding 1,000 rows on a table of 10,000 rows (no warmup).
* clear rows: Duration to clear the table filled with 10,000 rows. (no warmup)
* ready memory: Memory usage after page load.
* run memory: Memory usage after adding 1,000 rows.
* update memory: Memory usage after clicking 5 times update for a table with 1,000 rows.
* replace memory: Memory usage after clicking 5 times create 1,000 rows.
* repeated clear memory: Memory usage after creating and clearing 1,000 rows for 5 times.
* update memory: Memory usage after clicking 5 times update for a table with 1,000 rows.
* startup time: Duration for loading and parsing the javascript code and rendering the page.
* consistently interactive: The lighthouse metric TimeToConsistentlyInteractive: A pessimistic TTI - when the CPU and network are both definitely very idle. (no more CPU tasks over 50ms)
* script bootup time: The lighthouse metric ScriptBootUpTtime: The total ms required to parse/compile/evaluate all the page's scripts
* main thread work cost: The lighthouse metric MainThreadWorkCost: Total amount of time spent doing work on the main thread. includes style/layout/etc.
* total byte weight: The lighthouse metric TotalByteWeight: Network transfer cost (post-compression) of all the resources loaded into the page.

For all benchmarks the duration is measured including rendering time. You can read some details on this [article](http://www.stefankrause.net/wp/?p=218).

## How to get started - building and running

There are 3 framework entries in this repository:

* `keyed/wasm-bindgen`
* `keyed/yew`
* `keyed/yew-baseline` _(Used to determine if recent changes improved performance)_

### 1. Prerequisites

Have *node.js (>=10.0)* installed. We recommend using `nvm` to manage node versions The benchmark has been tested with node v12.16.2.
Please make sure that the following command work before trying to build:
```
> npm
npm -version
6.14.15
> nvm current
v12.16.2
> node --version
v12.16.2
```

### Install `chromedriver`

https://chromedriver.chromium.org/downloads


### Setup local server

You can use any server which properly handles the MIME type of wasm files. We recommend `miniserve`:

```
cargo install miniserve
```

Start `miniserve` in the root directory
```
miniserve .
```

Verify that the server works:
Try to open [http://localhost:8080/index.html](http://localhost:8080/index.html). If you see something like that you're on the right track:
![Index.html](images/index.png?raw=true "Index.html")

Now open a new terminal window and keep `miniserve` running in background.

### 2. Building and running a single framework

We now try to build a framework. Go to the `wasm-bindgen` reference implementation
```
cd frameworks/keyed/wasm-bindgen
```
and install the dependencies
```
npm install
```
and build the framework
```
npm run build-prod
```
There should be no build errors and we can open the framework in the browser:
[http://localhost:8080/frameworks/keyed/wasm-bindgen/index.html](http://localhost:8080/frameworks/keyed/wasm-bindgen/index.html)

## 3. Running a single framework with the automated benchmark driver

The benchmark uses an automated benchmark driver using `chromedriver` to measure the duration for each operation using chrome's timeline. Here are the steps to run for a single framework:

### Run benchmarks

First, navigate to the `webdriver-ts/` directory.

and install the dependencies
```
npm install
```
and build the benchmark driver
```
npm run build-prod
```
now run the benchmark driver for the "wasmbindgen-keyed" framework:
```
npm run bench -- --headless keyed/wasm-bindgen
```
Just lean back and wait for chrome to run the benchmarks. 
Keep an eye on the console output to see if benchmark checks are timing out. If so, visit [http://localhost:8080/frameworks/keyed/wasm-bindgen/index.html](http://localhost:8080/frameworks/keyed/wasm-bindgen/index.html) and check for console errors. You could also try without using `--headless` to debug.

The results for that run will be saved in the `webdriver-ts/results` directory. We can take a look at the results of a single result:
```
cat results/wasm-bindgen-v0.2.47-keyed_01_run1k.json

{
  "framework": "wasm-bindgen-v0.2.47-keyed",
  "keyed": true,
  "benchmark": "01_run1k",
  "type": "cpu",
  "min": 101.123,
  "max": 114.547,
  "mean": 107.69300000000001,
  "median": 105.821,
  "geometricMean": 107.5587774247631,
  "standardDeviation": 6.028027040417119,
  "values": [
    114.547,
    113.509,
    103.465,
    105.821,
    101.123
  ]
}
```
As you can see the mean duration for create 1000 rows was 107 msecs.

You can also check whether the implementation appears to be compliant to the rules:
```
npm run check keyed/wasm-bindgen
```
If it finds anything it'll report an ERROR.

## 4. Building the result table

Install libraries
```
(cd webdriver-ts-results && npm install)
```

In the webdriver-ts directory issue the following command:
```
npm run results
```
Now a result table should have been created which can be opened on [http://localhost:8080/webdriver-ts-results/table.html](http://localhost:8080/webdriver-ts-results/table.html).

## 5. Building and running the benchmarks for all frameworks

From the base directory, you can build all frameworks by running
```
npm install
npm run build-prod
```

You can now run the benchmark for all frameworks by:

1. Navigate to the `webdriver-ts/` directory
2. Run benches: `npm run bench -- --headless --count <COUNT>`
3. Process results: `npm run results`

After that, you can check all results in [http://localhost:8080/webdriver-ts/table.html](http://localhost:8080/webdriver-ts/table.html).

## Tips and tricks

* You can run multiple frameworks by passing their directory names (cd to webdriver-ts):
`npm run bench keyed/yew-baseline keyed/yew` or if you want to pass more options it becomes: 
`npm run bench -- --count 3 keyed/yew`.
* You can run all of the frameworks you've installed using `npm run bench -- --installed`
* To achieve good precision you should run each framework often enough. I recommend at least 10 times, more is better. The result table contains the mean and the standard deviation. You can seen the effect on the latter pretty well if you increase the count.

