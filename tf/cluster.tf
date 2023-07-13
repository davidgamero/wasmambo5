resource "azurerm_resource_group" "rg" {
  name = "battlesnake"

  // location based on cost https://azureprice.net/vm/Standard_B2s
  // and Battlesnake engine regions https://docs.battlesnake.com/guides/tips/engine-regions#currently-supported-engine-regions
  location = "East US"
}

resource "azurerm_kubernetes_cluster" "cluster" {
  name                      = "cluster"
  location                  = azurerm_resource_group.rg.location
  resource_group_name       = azurerm_resource_group.rg.name
  sku_tier                  = "Free"
  automatic_channel_upgrade = "stable"
  dns_prefix                = "battlesnake"

  default_node_pool {
    name                = "default"
    node_count          = 1
    enable_auto_scaling = false
    vm_size             = "Standard_B2s"
    os_disk_size_gb     = 30
  }

  identity {
    type = "SystemAssigned"
  }

  network_profile {
    load_balancer_sku = "standard" // basic doesn't support multiple node pools
    outbound_type     = "loadBalancer"
    network_plugin    = "kubenet"
  }
}

resource "azurerm_kubernetes_cluster_node_pool" "wasm" {
  name                  = "wasm"
  kubernetes_cluster_id = azurerm_kubernetes_cluster.cluster.id
  mode                  = "User"
  vm_size               = "Standard_B2s"
  node_count            = 1
  enable_auto_scaling   = false
  workload_runtime      = "WasmWasi"
  os_disk_size_gb       = 30
}
