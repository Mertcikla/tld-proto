.PHONY: generate push lint breaking

## Regenerate canonical artifacts from proto sources.
## For local wrapper/core development prefer `cd .. && make dev-proto`, which
## writes Go overlays to ../.dev/proto/go and TypeScript to
## ../frontend-wrapper/src/gen plus ../tld/frontend/src/gen.
generate:
	buf generate

## Lint all proto files.
lint:
	buf lint

## Check for breaking changes against the last BSR push.
breaking:
	buf breaking --against 'https://buf.build/tldiagramcom/diagram'

## Push to BSR. Always run `generate` first so the committed gen/ matches.
push: generate
	buf push
