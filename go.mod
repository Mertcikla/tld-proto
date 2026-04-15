module github.com/Mertcikla/tld-proto

go 1.26.1

tool (
	connectrpc.com/connect/cmd/protoc-gen-connect-go
	google.golang.org/protobuf/cmd/protoc-gen-go
)

require (
	buf.build/gen/go/tldiagramcom/diagram/protocolbuffers/go v1.36.11-20260414113744-f49e221fed77.1
	connectrpc.com/connect v1.19.1
	google.golang.org/protobuf v1.36.11
)
