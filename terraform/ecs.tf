resource "aws_ecs_cluster" "comments" {
  name = "${local.service}-cluster"
}

resource "aws_iam_role" "ecs" {
  name = "${local.service}-execution-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Sid    = ""
        Principal = {
          Service = "ecs-tasks.amazonaws.com"
        }
      },
    ]
  })
}

resource "aws_iam_role_policy_attachment" "ecs_task_execution" {
  role       = aws_iam_role.ecs.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

resource "aws_ecs_task_definition" "comments" {
  family                   = "${local.service}-task-family"
  network_mode             = "awsvpc"
  execution_role_arn       = aws_iam_role.ecs.arn
  cpu                      = 256
  memory                   = 2048
  requires_compatibilities = ["FARGATE"]

  container_definitions = templatefile("./app.json.tpl", {
    aws_ecr_repository = aws_ecr_repository.repo.repository_url
    tag                = "latest"
    region             = "eu-west-2"
    service            = "${local.service}"
    envvars            = var.envvars
    port               = var.port
    db_uri             = "postgres://${var.db_username}:${var.db_password}@${aws_db_instance.comments.address}/${aws_db_instance.comments.db_name}"
  })

  tags = {
    Application = "${local.service}"
  }
}

resource "aws_ecs_service" "comments" {
  name            = "${local.service}-service"
  cluster         = aws_ecs_cluster.comments.id
  task_definition = aws_ecs_task_definition.comments.arn
  desired_count   = 1
  launch_type     = "FARGATE"

  network_configuration {
    security_groups  = [aws_security_group.ecs_tasks.id]
    subnets          = data.aws_subnets.default.ids
    assign_public_ip = true
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.comments.arn
    container_name   = local.service
    container_port   = var.port
  }

  depends_on = [aws_lb_listener.https_forward, aws_iam_role_policy_attachment.ecs_task_execution]

  tags = {
    Application = "${local.service}"
  }
}

resource "aws_security_group" "ecs_tasks" {
  name        = "${local.service}-ecs_tasks-sg"
  description = "Allow access from the ALB only on the container port"

  ingress {
    protocol        = "tcp"
    from_port       = var.port
    to_port         = var.port
    security_groups = [aws_security_group.lb.id]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "null_resource" "push" {
  provisioner "local-exec" {
    command     = "./push.sh ${var.source_path} ${aws_ecr_repository.repo.repository_url} latest ${data.aws_caller_identity.current.account_id} ${local.service}"
    interpreter = ["bash", "-c"]
  }
}
