# GitHub Pages Setup Guide

To enable automatic deployment of your Docusaurus documentation to GitHub Pages, follow these steps:

## 1. Enable GitHub Pages

1. Go to your repository on GitHub: https://github.com/lambdv/aminus
2. Click on **Settings** tab
3. Scroll down to **Pages** section in the left sidebar
4. Under **Source**, select **GitHub Actions**
5. Click **Save**

## 2. Configure GitHub Pages Settings

- **Source**: GitHub Actions
- **Branch**: This will be automatically managed by the GitHub Actions workflow
- **Custom domain** (optional): Leave blank unless you have a custom domain

## 3. Verify Deployment

After pushing changes to the `main` branch:

1. Go to the **Actions** tab in your repository
2. You should see the "Deploy Docusaurus to GitHub Pages" workflow running
3. Once completed, your site will be available at: https://lambdv.github.io/aminus/

## 4. Troubleshooting

### If the workflow fails:

1. Check the **Actions** tab for error messages
2. Common issues:
   - Node.js version compatibility
   - Missing dependencies
   - Build errors in Docusaurus

### If the site doesn't appear:

1. Wait a few minutes for GitHub Pages to update
2. Check if the `github-pages` environment is properly configured
3. Verify the base URL in `docusaurus.config.ts` is set to `/aminus/`

## 5. Environment Variables (if needed)

The workflow uses the default `GITHUB_TOKEN` which should have the necessary permissions. If you encounter permission issues:

1. Go to **Settings** > **Actions** > **General**
2. Under **Workflow permissions**, select **Read and write permissions**
3. Save the changes

## 6. Custom Domain (Optional)

If you want to use a custom domain:

1. Add your domain to the **Custom domain** field in GitHub Pages settings
2. Update the `url` in `docusaurus.config.ts` to match your domain
3. Update the `baseUrl` to `/` if using a custom domain
4. Add a CNAME file to the `static/` directory with your domain name

## 7. Monitoring Deployments

- Check deployment status in the **Actions** tab
- View deployment logs for debugging
- Monitor the **Environments** section for deployment history 