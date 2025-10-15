FROM ghcr.io/open-education-hub/openedu-builder:0.5.1

# Install tools.
RUN apt-get update && \
    apt-get install -yqq ffmpeg curl make

# Install MarkdownPP using pip.
RUN pip install MarkdownPP
# Verify installation
RUN which markdown-pp || echo "markdown-pp not found in PATH"
RUN python -c "import markdownpp; print('MarkdownPP module available')"

# Install node LTS (16)
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get update && \
    apt-get install -yqq nodejs

# Install reveal-md using npm.
RUN npm install -g reveal-md

# Install Docusaurus.
RUN npm install create-docusaurus@2.1.0

WORKDIR /content

ENTRYPOINT ["oe_builder"]
