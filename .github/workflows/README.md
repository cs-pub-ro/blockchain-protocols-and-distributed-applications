# GitHub Actions Workflows

This directory contains automated workflows for building and deploying course materials.

## Workflows

### 1. `build-slides.yml` - Lecture Slides Builder

Automatically builds and deploys lecture slides to GitHub Pages.

**Triggers:**
- Push to `main` branch (when lecture files change)
- Manual workflow dispatch

**What it does:**
1. Builds Docker image with reveal-md and markdown-pp
2. Automatically detects all lecture directories in `chapters/*/lecture/`
3. Builds slides using the Makefile in each lecture directory
4. Creates an index page for easy navigation
5. Deploys to `gh-pages` branch under `/slides/` directory

**Accessing the slides:**
- Main index: `https://[username].github.io/[repo-name]/slides/`
- Individual lectures: `https://[username].github.io/[repo-name]/slides/[chapter-name]/`

**Adding new lectures:**
Just create a new lecture directory with a Makefile following the existing pattern:
```
chapters/
  new-chapter/
    lecture/
      Makefile          # Must exist and support 'make html' target
      slides.mdpp       # Main slides file
      slides/
        topic.md        # Individual slide files
```

The workflow will automatically detect and build the new lecture on the next push.

### 2. `deployment.yml` - Full Site Deployment

Builds the complete Docusaurus site with embedded slides using the `config.yaml` configuration.

### 3. `actions.yml` - Linter Workflow

Runs code quality checks on pull requests.

## Configuration

### Dockerfile

The `Dockerfile` at the repository root contains all dependencies:
- Python 3 with MarkdownPP
- Node.js LTS with reveal-md
- Build tools (make, ffmpeg)

### GitHub Pages Setup

After the first successful workflow run:

1. Go to repository **Settings** → **Pages**
2. Under **Build and deployment**, select:
   - Source: **Deploy from a branch**
   - Branch: **gh-pages**
   - Folder: **/ (root)**
3. Click **Save**

Slides will be available at your GitHub Pages URL within a few minutes.

## Troubleshooting

### Workflow fails to build

1. Check the Actions tab for detailed error logs
2. Verify that `chapters/*/lecture/Makefile` exists and works locally
3. Ensure all slide files are properly formatted markdown

### Slides not appearing on GitHub Pages

1. Verify GitHub Pages is configured correctly (see above)
2. Check that the gh-pages branch was created
3. Wait a few minutes for GitHub Pages to update

### Local testing

To test the build locally before pushing:

```bash
cd chapters/introduction/lecture
make html
# Open _site/index.html in your browser
```

## Manual Workflow Trigger

You can manually trigger the slides build from the GitHub Actions tab:

1. Go to **Actions** → **Build and Deploy Slides**
2. Click **Run workflow**
3. Select the `main` branch
4. Click **Run workflow**

