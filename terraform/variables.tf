locals {
  service = "sm-comments"
}

variable "vpc_id" {}
variable "db_username" {}
variable "db_password" {}
variable "source_path" {
  type = string
  description = "path to dockerimage"
  default = ".."
}
variable "envvars" {
  type        = map(string)
  description = "container environment variables"
}
variable "port" {
  type        = number
  description = "container port"
  default     = 8000
}

data "aws_caller_identity" "current" {}

data "aws_vpc" "default" {
  default = true
}

data "aws_subnets" "default" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
}
