curl -X POST \
-H "Content-Type: application/json" \
-d '{
"id": 1,
"name": "John Doe",
"email": "john.doe@example.com",
"phone": "123-456-7890"
}' \
http://127.0.0.1:8080/customers

curl -X POST \
-H "Content-Type: application/json" \
-d '{
"id": 101,
"customer_id": 1,
"account_type": "Savings",
"balance": 5000.0
}' \
http://127.0.0.1:8080/accounts

curl -X GET http://127.0.0.1:8080/customers/1 | jq

curl -X GET http://127.0.0.1:8080/accounts/101 | jq

curl -X GET http://127.0.0.1:8080/all-data | jq
