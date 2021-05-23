#! /bin/bash
# Usage: ./bechmark_change.sh CHANGE_BRANCH

http -p 8080 &

(
	set -euo pipefail

	npm install -d

	(
		set -euo pipefail
		cd frameworks/keyed/yew
		rm -f Cargo.lock
		npm ci
		npm run build-prod
	)
	(
		set -euo pipefail
		cd frameworks/keyed/yew-baseline
		rm -f Cargo.lock
		npm ci
		npm run build-prod
	)
	(
		set -euo pipefail
		cd webdriver-ts-results
		npm ci
	)
	(
		set -euo pipefail
		cd webdriver-ts
		rm -rf results
		npm ci
		npm run build-prod
		npm run bench -- --headless keyed/yew keyed/yew-baseline
		npm run results
	)

	chromium http://localhost:8080/webdriver-ts-results/table.html
)

kill %%
