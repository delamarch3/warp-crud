resource "aws_cloudwatch_log_group" "comments" {
  name = "${local.service}-log-group"

  tags = {
    Application = "${local.service}"
  }
}
