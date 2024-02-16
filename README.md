# tide-rest-api

# Creating a simple applications in different languages and technologies.

## Requirements:

### Rest API Endpoints:

* /
  * Simple Hello world! ✅
* /api/redis/key
  * GET /:key -  Returns required key ❌
  * POST /set - Inserts key into redis ❌
  * GET /count - Returns count of all keys ❌
  * GET /count/pattern - Returns count of specified pattern ❌
* /api/Employee
  * GET /all - Returns all employees ❌
  * GET /:id - Returns employee with given id ❌
  * POST /new - Creates a new employee ❌
  * PUT /:id - Put to edit an employee ❌
  * DELETE /:id - Delete employee with given id ❌
* /health - Returns health of all applications and dependencies ❌

### Middlewares:
* Logger middleware - logging request into message bus ❌
* Monitoring middleware - Prometheus like statistics ❌

### Technologies:
* Redis ❌
* SQL DB ❌
* Docker ❌
* Jenkins ❌

### Optional implementations
* Generate swagger (json/yaml) ❌
* Implement SwaggerUI ❌

### Objects examples
* Redis-Key

´´´
{
  "key": string,
  "value": string,
  "expiration": u32
}
´´´

* Employee

´´´
{
  "id": string,
  "name": string,
  "position": string,
  "salary": u32,
  "managerId": string
}
´´´
