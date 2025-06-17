terraform {
  required_providers {
    http = {
      source  = "hashicorp/http"
      version = "~> 3.0"
    }
  }
}

provider "http" {}

# 1. Request a token to Anypoint
data "http" "getAnypointAccessToken" {
  url    = var.anypoint-token-url
  method = "POST"
  request_headers = {
    Content-Type = "application/x-www-form-urlencoded"
  }
  request_body = "grant_type=client_credentials&client_id=${var.client-id}&client_secret=${var.client-secret}"
}

# Define the local variable :
# 1- Parse the access token from the JSON response
# 2- Build the Gateway URL
locals {
  token_response = jsondecode(data.http.getAnypointAccessToken.response_body)
  access_token   = local.token_response.access_token
  gateway_url = "${var.gateway-baseapi-url}${var.bg-id}/environments/${var.env-id}/gateways"
}

# Output the token and API response
# output "access_token" {
# value = local.access_token
# }

# 2. List Gateway
# data "http" "gateway" {
#   url    = var.gateway-url
#   method = "GET"

#   request_headers = {
#     Authorization = "Bearer ${local.access_token}"
#   }
# }

# 2. List PS ID
# data "http" "ps-id" {
#   url    = var.target-url
#   method = "GET"

#   request_headers = {
#     Authorization = "Bearer ${local.access_token}"
#   }
# }

# 2. List DOMAINS ID
# data "http" "domain-id" {
#   url    = var.domainId-url
#   method = "GET"

#   request_headers = {
#     Authorization = "Bearer ${local.access_token}"
#   }
# }

# 3. Output result
# output "gateway" {
#   value = data.http.gateway.response_body
# }

# # 3. Output result
# output "ps-id" {
#   value = data.http.ps-id.response_body
# }

# # 3. Output result
# output "domain-id" {
#   value = data.http.domain-id.response_body
# }

# Gateway creation in SaaS
data "http" "post_gateway" {
  #url    = "http://localhost:8087/gtw"
  #url    = var.gateway-url
  url     = local.gateway_url
  method = "POST"

  request_headers = {
    Authorization = "Bearer ${local.access_token}"
    Content-Type = "application/json"
  }

  request_body = jsonencode({
    name= var.gateway-name
    targetId= var.gateway-privatespace-id
    runtimeVersion = var.gateway-runtimeVersion
    releaseChannel = var.gateway-realsechannel
    size = var.gateway-size
    configuration = {

        ingress = {
            publicUrl = "https://${var.gateway-name}${var.gateway-dns}"
            lastMileSecurity = var.gateway-lastMileSecurity
            forwardSslSession = var.gateway-forwardSslSession
        }
        properties = {
                upstreamResponseTimeout = var.gateway-upstreamResponseTimeout
                connectionIdleTimeout = var.gateway-connectionIdleTimeout
        }
        logging = {
          level = var.gateway-level-log
          forwardLogs = var.gateway-forward-logs
        }
        tracing =  {
            enabled = false
        }
    }
  })
}

# 3. Output result
output "post_gateway" {
  value = data.http.post_gateway.response_body
}
