grpcurl -plaintext --proto ./proto/calculator.proto \
 -d '{"a": 10, "b": 20}' \
 '[::1]:50051' calculator.Calculator.Add
