[
    {
      "name": "${service}",
      "image": "${aws_ecr_repository}:${tag}",
      "essential": true,
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-region": "${region}",
          "awslogs-stream-prefix": "${service}",
          "awslogs-group": "${service}-log-group"
        }
      },
      "portMappings": [
        {
          "containerPort": ${port},
          "hostPort": ${port},
          "protocol": "tcp"
        }
      ],
      "cpu": 1,
      "environment": [
        %{ for key, value in envvars }
        {
          "name": "${key}",
          "value": "${value}"
        },
        %{ endfor ~}
        {
          "name": "DB_URI",
          "value": "${db_uri}"
        }
      ],
      "ulimits": [
        {
          "name": "nofile",
          "softLimit": 65536,
          "hardLimit": 65536
        }
      ],
      "mountPoints": [],
      "memory": 2048,
      "volumesFrom": []
    }
  ]
  
  
  