FROM ghcr.io/open-education-hub/openedu-builder:0.6.1

# Install tools.
RUN apt-get update && \
    apt-get install -yqq ffmpeg curl make

# Install MarkdownPP using pip.
RUN pip install MarkdownPP
# Verify installation
RUN which markdown-pp || echo "markdown-pp not found in PATH"

# Install Node 20 (Docusaurus 2.1.0 is not compatible with newer Node versions)
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get update && \
    apt-get install -yqq nodejs

# Install reveal-md using npm.
RUN npm install -g reveal-md

# Install Docusaurus.
RUN npm install create-docusaurus@2.1.0

# Docusaurus 2.1.0 breaks with current webpack (5.111.1 lacks ContextReplacementPlugin,
# newer versions reject webpackbar's ProgressPlugin options). The site is installed at
# runtime by oe_builder (npx create-docusaurus), so ignore npm packages published after
# the Docusaurus 2.1.0 era. Keep this after the reveal-md install.
ENV NPM_CONFIG_BEFORE=2022-11-01

WORKDIR /content

ENTRYPOINT ["oe_builder"]
