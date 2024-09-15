# Arsis

```yaml
kubectl create secret generic arsis-secret --from-literal=discord-token=<TOKEN>
kubectl apply -f kubernetes/role-binding.yaml
kubectl apply -f kubernetes/deployment.yaml
```
