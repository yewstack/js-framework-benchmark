#! /bin/bash
# Usage: ./bechmark_change.sh CHANGE_BRANCH

git reset --hard
sed -i 's/PLACEHOLDER/'"${1/\//\\\/}"'/' frameworks/keyed/yew/Cargo.toml

http -p 8080 &

(
	set -euo pipefail

	npm install -d

	(
		set -euo pipefail
		cd frameworks/keyed/wasm-bindgen
		npm install -d
		npm run build-prod
	)
	(
		set -euo pipefail
		cd frameworks/keyed/yew
		npm install -d
		npm run build-prod
	)
	(
		set -euo pipefail
		cd frameworks/keyed/yew-baseline
		npm install -d
		npm run build-prod
	)
	(
		set -euo pipefail
		cd webdriver-ts-results
		npm install -d
	)
	(
		set -euo pipefail
		cd webdriver-ts
		npm install -d
		npm run build-prod
		npm run bench -- --headless \
			frameworks/keyed/wasm-bindgen \
			keyed/yew keyed/yew-baseline
		npm run results
	)

	chromium http://localhost:8080/webdriver-ts-results/table.html
)

kill %%
