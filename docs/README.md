# Aminus Documentation

This directory contains the Docusaurus documentation for the Aminus project.

## Local Development

To run the documentation site locally:

```bash
# Install dependencies
npm install

# Start the development server
npm start

# Build for production
npm run build

# Serve the production build locally
npm run serve
```

## Deployment

The documentation is automatically deployed to GitHub Pages when changes are pushed to the `main` branch. The deployment is handled by the GitHub Actions workflow in `.github/workflows/deploy-docs.yml`.

### Deployment Process

1. When you push changes to the `main` branch, the GitHub Actions workflow automatically:
   - Sets up Node.js environment
   - Installs dependencies
   - Builds the Docusaurus site
   - Deploys to GitHub Pages

2. The site will be available at: https://lambdv.github.io/aminus/

### Manual Deployment

If you need to deploy manually:

1. Build the site: `npm run build`
2. The built files will be in the `build/` directory
3. You can then deploy these files to any static hosting service

## Configuration

The main configuration file is `docusaurus.config.ts`. Key settings:

- **URL**: https://lambdv.github.io
- **Base URL**: /aminus/
- **Organization**: lambdv
- **Project**: aminus

## Adding Content

- **Documentation**: Add markdown files to the `docs/` directory
- **Blog Posts**: Add markdown files to the `blog/` directory
- **Pages**: Add React components to the `src/pages/` directory
- **Components**: Add React components to the `src/components/` directory

## Customization

- **Styling**: Modify `src/css/custom.css`
- **Theme**: Update the `themeConfig` in `docusaurus.config.ts`
- **Sidebar**: Edit `sidebars.ts` to configure the documentation navigation
