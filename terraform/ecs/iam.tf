data "aws_iam_policy_document" "assume_role_policy" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["ecs-tasks.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "ecs_task_execution_role" {
  name               = "${var.app_name}_ecs_task_execution_role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json
}

resource "aws_iam_role_policy_attachment" "ecs_task_execution_role_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

# CloudWatch Access
resource "aws_iam_role_policy_attachment" "cloudwatch_write_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/CloudWatchLogsFullAccess"
}

# Prometheus Access
resource "aws_iam_role_policy_attachment" "prometheus_write_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonPrometheusRemoteWriteAccess"
}

# Analytics Bucket Access
resource "aws_iam_policy" "analytics_bucket_access" {
  name        = "${var.app_name}_analytics_bucket_access"
  path        = "/"
  description = "Allows ${var.app_name} to read/write from ${var.analytics_bucket_name}"

  # Terraform's "jsonencode" function converts a
  # Terraform expression result to valid JSON syntax.
  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Sid" : "ListObjectsInAnalyticsBucket",
        "Effect" : "Allow",
        "Action" : ["s3:ListBucket"],
        "Resource" : ["arn:aws:s3:::${var.analytics_bucket_name}"]
      },
      {
        "Sid" : "AllObjectActionsInAnalyticsBucket",
        "Effect" : "Allow",
        "Action" : ["s3:CopyObject", "s3:DeleteObject", "s3:GetObject", "s3:HeadObject", "s3:PutObject", "s3:RestoreObject"],
        "Resource" : ["arn:aws:s3:::${var.analytics_bucket_name}/*"]
      },
      {
        "Sid" : "ListObjectsInGeoipBucket",
        "Effect" : "Allow",
        "Action" : ["s3:ListBucket"],
        "Resource" : ["arn:aws:s3:::${var.analytics_geoip_db_bucket_name}"]
      },
      {
        "Sid" : "AllObjectActionsInGeoipBucket",
        "Effect" : "Allow",
        "Action" : ["s3:CopyObject", "s3:GetObject", "s3:HeadObject"],
        "Resource" : ["arn:aws:s3:::${var.analytics_geoip_db_bucket_name}/*"]
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "bucket-policy-attach" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = aws_iam_policy.analytics_bucket_access.arn
}
