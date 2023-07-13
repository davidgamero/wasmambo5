output "resource_group_name" {
  value = azurerm_resource_group.rg.name
}

output "kubernetes_cluster_name" {
  value = azurerm_kubernetes_cluster.cluster.name
}

output "tenant_id" {
  value = data.azuread_client_config.current.tenant_id
}

output "client_id" {
  value = azuread_application.app.application_id
}

output "subscription_id" {
  value = data.azurerm_subscription.current.subscription_id
}
