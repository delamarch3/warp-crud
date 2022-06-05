resource "aws_security_group" "lb" {
  name        = "${local.service}-lb-sg"
  description = "Public http access to the ALB"

  ingress {
    protocol    = "tcp"
    from_port   = 80
    to_port     = 80
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_lb" "comments" {
  name               = "${local.service}-alb"
  subnets            = data.aws_subnets.default.ids
  load_balancer_type = "application"
  security_groups    = [aws_security_group.lb.id]

  tags = {
    Application = "${local.service}"
  }
}

resource "aws_lb_listener" "https_forward" {
  load_balancer_arn = aws_lb.comments.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.comments.arn
  }
}

# Attached to the ECS service resource:
resource "aws_lb_target_group" "comments" {
  name        = "${local.service}-alb-tg"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = data.aws_vpc.default.id
  target_type = "ip"

  health_check {
    healthy_threshold   = "3"
    interval            = "90"
    protocol            = "HTTP"
    matcher             = "200-299"
    timeout             = "20"
    path                = "/health"
    unhealthy_threshold = "2"
  }
}

output "lb_dns" {
  value = aws_lb.comments.dns_name
}
