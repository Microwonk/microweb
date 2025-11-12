ASSETS_DIR = public/assets
METADATA = cv/metadata.toml
TMP_METADATA = cv/metadata.tmp.toml

watch-css:
	npx tailwindcss -i ./input.css -o ./style/main.css --watch

local-db:
	docker compose -f database-compose.yml up -d

watch:
	cargo leptos watch

build:
	cargo leptos build --release

package: cv build

cv-en:
	cp $(METADATA) $(TMP_METADATA)
	sed 's/^language = ".*"$$/language = "en"/' $(TMP_METADATA) > $(METADATA)
	typst c cv/cv.typ $(ASSETS_DIR)/CV_EN.pdf
	mv $(TMP_METADATA) $(METADATA)

cv-de:
	cp $(METADATA) $(TMP_METADATA)
	sed 's/^language = ".*"$$/language = "de"/' $(TMP_METADATA) > $(METADATA)
	typst c cv/cv.typ $(ASSETS_DIR)/CV_DE.pdf
	mv $(TMP_METADATA) $(METADATA)

cv: cv-de cv-en