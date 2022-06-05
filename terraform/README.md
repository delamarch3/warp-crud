# tfvars

-   db_username = ""
-   db_password = ""
-   vpc_id = ""
-   envvars = {}

# Resources

## RDS

## Security groups

-   SG for ALB to allow http
-   SG for ECS service task to allow access from ALB
-   SG for RDS to allow access from ECS service

## Load balancer

-   Application load balancer
-   Load balancer listener
-   Load balancer target group

## Elastic container registry

-   ECR repo
-   ECR lifecycle policy

## Elastic container service

-   ECS cluster
-   ECS task execution iam policy + role
-   ECS task definiton (app.json.tpl)
-   ECS service

## Cloadwatch

-   Log group
