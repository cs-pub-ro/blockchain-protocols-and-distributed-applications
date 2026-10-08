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

# webpack 5.111.1 (published 2026-09-18) is missing lib/ContextReplacementPlugin.js,
# which breaks Docusaurus 2.1.0. Ignore npm packages published after this date.
ENV NPM_CONFIG_BEFORE=2026-09-15

# Install reveal-md using npm.
RUN npm install -g reveal-md

# Install Docusaurus.
RUN npm install create-docusaurus@2.1.0

WORKDIR /content

ENTRYPOINT ["oe_builder"]
