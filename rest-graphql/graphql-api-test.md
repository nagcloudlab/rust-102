curl http://127.0.0.1:8080/graphql-schema

curl -X POST \
-H "Content-Type: application/json" \
-d '{
"query": "mutation { addCustomer(input: { id: 1, name: \"John Doe\", email: \"john.doe@example.com\", phone: \"123-456-7890\" }) { id name email phone } }"
}' \
http://127.0.0.1:8080/graphql

curl -X POST \
-H "Content-Type: application/json" \
-d '{
"query": "mutation { addAccount(input: { id: 101, customerId: 1, accountType: \"Savings\", balance: 5000.0 }) { id customerId accountType balance } }"
}' \
http://127.0.0.1:8080/graphql

curl -X POST \
-H "Content-Type: application/json" \
-d '{
"query": "query { getCustomer(id: 1) { id name email phone } }"
}' \
http://127.0.0.1:8080/graphql | jq

curl -X POST \
-H "Content-Type: application/json" \
-d '{
"query": "query { getAccount(id: 101) { id customerId accountType balance } }"
}' \
http://127.0.0.1:8080/graphql | jq
