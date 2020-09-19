#! /bin/bash
# Usage: ./bechmark_change.sh CHANGE_BRANCH

sed -i 's/PLACEHOLDER/'"${1/\//\\\/}"'/' frameworks/keyed/yew/Cargo.toml

http -p 8080 &

(
	set -euo pipefail

	npm install -d
	(cd frameworks/keyed/wasm-bindgen &&
		npm run build-prod)
	(cd frameworks/keyed/yew &&
		npm install -d &&
		npm run build-prod)
	(cd frameworks/keyed/yew-baseline &&
		npm install -d &&
		npm run build-prod)

	(cd webdriver-ts-results &&
		npm install -d)
	(cd webdriver-ts &&
		npm install -d &&
		npm run build-prod &&
		npm run bench -- --headless keyed/yew keyed/yew-baseline &&
		npm run results)

	chromium http://localhost:8080/webdriver-ts-results/table.html
)

kill %%
