# Build

cargo concordium build --out dist/module.wasm.v1 --schema-out dist/schema.bin

# Deploy

concordium-client module deploy dist/module.wasm.v1 --sender <YOUR-ADDRESS> --name counter --grpc-port 10001

# Initialize the contract

concordium-client contract init <YOUR-MODULE-HASH> --sender <YOUR-ADDRESS> --energy 30000 --contract counter --grpc-port 10001

# Increment

concordium-client contract update <YOUR-CONTRACT-INSTANCE> --entrypoint mint --parameter-json <PATH-TO-JSON> --schema dist/schema.bin --sender <YOUR-ADDRESS> --energy 6000 --grpc-port 10001

# View state

concordium-client contract invoke <YOUR-CONTRACT-INSTANCE> --entrypoint view --schema dist/schema.bin --grpc-port 10001
