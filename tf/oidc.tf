// https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure?tabs=azure-cli%2Clinux#create-an-azure-active-directory-application-and-service-principal

data "azuread_client_config" "current" {}

data "azurerm_subscription" "current" {}


resource "azuread_application" "app" {
  display_name = "battlesnake"
}


resource "azuread_service_principal" "sp" {
  application_id               = azuread_application.app.application_id
}

// https://learn.microsoft.com/en-us/azure/role-based-access-control/built-in-roles#azure-kubernetes-service-cluster-user-role
resource "azurerm_role_assignment" "cluster" {
  scope                = azurerm_kubernetes_cluster.cluster.id
  role_definition_name = "Azure Kubernetes Service Cluster User Role"
  principal_id         = azuread_service_principal.sp.id
}

resource "azuread_application_federated_identity_credential" "github" {
  application_object_id = azuread_application.app.object_id
  display_name          = "github"
  audiences             = ["api://AzureADTokenExchange"]
  issuer                = "https://token.actions.githubusercontent.com"
  subject               = "repo:davidgamero/wasmambo5:environment:production"
}
