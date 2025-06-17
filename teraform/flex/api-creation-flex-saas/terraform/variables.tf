
variable "bg-id" {
  description = "Anypoint Business group ID" 
  type        = string
  sensitive   = true
}

variable "env-id" {
  description = "Anypoint Environnment ID" 
  type        = string
  sensitive   = true
}

variable "gateway-baseapi-url" {
  description = "Gateway Anypoint API URL" 
  type        = string
  default     = "https://eu1.anypoint.mulesoft.com/gatewaymanager/api/v1/organizations/"
}

variable "anypoint-token-url" {
  description = "Anypoint API token url"
  type        = string
  default     = "https://eu1.anypoint.mulesoft.com/accounts/api/v2/oauth2/token"
}

variable "target-url" {
  description = "Private-Space Flex API url"
  type        = string
}

variable "gateway-url" {
  description = "FLEX API url"
  type        = string
}

variable "domainId-url" {
  description = "Anypoint API token url"
  type        = string
}

variable "client-id" {
  description = "Anypoint Client Secret for connexion"
  type        = string
  sensitive   = true
}

variable "client-secret" {
  description = "Anypoint Client Secret for connexion"
  type        = string
  sensitive   = true
}

variable "gateway-runtimeVersion" {
  description = "Runtime version"
  type        = string
  default = "1.9.3"
}

variable "gateway-realsechannel" {
  description = "Gateway Release type"
  type        = string
  default = "lts"
}

variable "gateway-size" {
  description = "Gateway Size"
  type        = string
  default = "small"
}

variable "gateway-lastMileSecurity" {
  description = "Lastime Security boolean"
  type        = bool
  default = false
}

variable "gateway-forwardSslSession" {
  description = "Forward the SSL offloading"
  type        = bool
  default = false
}

variable "gateway-upstreamResponseTimeout" {
  description = "TimeOut for the Upstream"
  type        = number
  default = 15
}

variable "gateway-connectionIdleTimeout" {
  description = "Idle Time"
  type        = number
  default = 60
}

variable "gateway-level-log" {
  description = "Level Log of the Runtime"
  type        = string
  default = "info"
}

variable "gateway-forward-logs" {
  description = "Allow to forward the log to Anypoint Monitoring"
  type        = bool
  default = true
}

variable "gateway-name" {
  description = "Name of Gateway instance"
  type        = string
  default = "gtw"
}

variable "gateway-privatespace-id" {
  description = "Id of the Private Space"
  type        = string
}

variable "gateway-dns" {
  description = "Wilcard DNS"
  type        = string
}








