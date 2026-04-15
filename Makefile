.PHONY: generate push lint breaking

## Regenerate Go (grpc/gen/) and TypeScript (../frontend/src/gen/) from proto sources.
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
