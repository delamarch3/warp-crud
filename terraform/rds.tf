resource "aws_db_instance" "comments" {
  allocated_storage    = 10
  identifier           = local.service
  engine               = "postgres"
  engine_version       = "13.4"
  instance_class       = "db.t3.micro"
  db_name              = "sm_comments"
  username             = var.db_username
  password             = var.db_password
  port                 = 5432
  parameter_group_name = "default.postgres13"
  skip_final_snapshot  = true

  # Set to false once null_resource db_setup has provisioned
  publicly_accessible  = true

  vpc_security_group_ids = [aws_security_group.rds.id]
}

resource "aws_security_group" "rds" {
  name   = "${local.service}-rds-sg"
  vpc_id = data.aws_vpc.default.id

  # Comment out once null_resource db_setup has provisioned
  ingress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }
  
  # Only allow inbound traffic from ecs service:
  ingress {
    from_port        = 5432
    to_port          = 5432
    protocol         = "TCP"
    security_groups = [aws_security_group.ecs_tasks.id]
  }

  egress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }

  tags = {
    Name = "${local.service}-rds"
  }
}

# Creates table:
resource "null_resource" "db_setup" {
  depends_on = [aws_db_instance.comments, aws_security_group.rds]

  triggers = { arn = aws_db_instance.comments.arn }
  provisioner "local-exec" {
    command = <<EOF
    psql ${aws_db_instance.comments.db_name} -h ${aws_db_instance.comments.address} \
    -p ${aws_db_instance.comments.port} -U ${var.db_username} < ../database/init.sql
    EOF
    environment = {
      PGPASSWORD = var.db_password
    }
  }
}

output "address" {
  value = aws_db_instance.comments.address
}
