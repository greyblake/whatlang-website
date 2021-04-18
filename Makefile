all: build_artifacts purge_public create_public copy_artifacts

purge_public:
	rm -rf ./public

create_public:
	mkdir public

build_artifacts:
	cargo make verify
	cargo make build_release

copy_artifacts:
	cp ./index.html ./public/
	cp ./assets/favicon.ico ./public/
	cp -r ./assets/ ./public/


#
# [2021-04-18 11:47:16] - 127.0.0.1 - 200 - GET /favicon.ico
# [2021-04-18 11:47:18] - 127.0.0.1 - 200 - GET /index.html
# [2021-04-18 11:47:18] - 127.0.0.1 - 200 - GET /assets/bulmaswatch-lumen.min.css
# [2021-04-18 11:47:18] - 127.0.0.1 - 200 - GET /assets/whatlang-logo.png
# [2021-04-18 11:47:18] - 127.0.0.1 - 200 - GET /pkg/package.js
# [2021-04-18 11:47:18] - 127.0.0.1 - 200 - GET /pkg/package_bg.wasm
