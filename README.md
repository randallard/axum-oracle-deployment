# axum-oracle-deployment
run these first

    kubectl patch ingress main-ingress --patch-file k8s\base\ingress-patch.yaml
    kubectl apply -f .\k8s\base\deployment.yaml
    kubectl apply -f .\k8s\base\service.yaml


## set up the GitHub Actions permissions and secrets

1. First, enable GitHub Actions for the repository:
   - Go to your repository Settings
   - Navigate to "Actions" under "Code and automation"
   - Enable "Allow all actions and reusable workflows"

2. Set up access to GitHub Container Registry:
   - Go to your GitHub profile Settings
   - Navigate to "Developer settings" > "Personal access tokens" > "Tokens (classic)"
   - Create a new token with these permissions:
     - `read:packages`
     - `write:packages`
     - `delete:packages`
   - Save this token value as a repository secret named `CR_PAT`:
     - Go to your repository Settings
     - Navigate to "Secrets and variables" > "Actions"
     - Click "New repository secret"
     - Name: `CR_PAT`
     - Value: your token from above

3. Set up Oracle Cloud secrets (for the OCI CLI):
   - Create a secret named `OCI_CONFIG` with your OCI config file contents, typically found in `~/.oci/config`. It looks like:
     ```ini
     [DEFAULT]
     user=ocid1.user.oc1..aaaaaaaa...
     fingerprint=xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx
     tenancy=ocid1.tenancy.oc1..aaaaaaaa...
     region=us-phoenix-1
     key_file=~/.oci/key.pem
     ```
   - Create a secret named `OCI_PRIVATE_KEY` with your OCI API private key contents (the private key associated with the fingerprint in your config)

4. Set up Kubernetes config:
   - Create a secret named `KUBE_CONFIG` with your kubeconfig file contents for the Oracle Kubernetes cluster
   - You can get this from your Oracle Cloud console or by running:
     ```bash
     oci ce cluster create-kubeconfig --cluster-id <your-cluster-id> --file - --token-version 2.0.0
     ```

5. Enable workflow permissions in the repository:
   - Go to repository Settings
   - Navigate to "Actions" > "General"
   - Under "Workflow permissions", select:
     - "Read and write permissions"
     - Check "Allow GitHub Actions to create and approve pull requests"

Here's a checklist of all required secrets:
```
✓ CR_PAT           (for GitHub Container Registry)
✓ OCI_CONFIG       (Oracle Cloud CLI config)
✓ OCI_PRIVATE_KEY  (Oracle Cloud API private key)
✓ KUBE_CONFIG      (Kubernetes cluster config)
```

