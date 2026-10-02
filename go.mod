module github.com/Mertcikla/tld-proto

go 1.26.1

tool (
	connectrpc.com/connect/cmd/protoc-gen-connect-go
	google.golang.org/protobuf/cmd/protoc-gen-go
)

require (
	buf.build/gen/go/tldiagramcom/diagram/protocolbuffers/go v1.36.12-20261001025305-f90b1fd4390d.2
	connectrpc.com/connect v1.21.0
	google.golang.org/protobuf v1.36.12
)
