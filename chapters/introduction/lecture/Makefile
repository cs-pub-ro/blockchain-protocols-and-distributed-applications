RVMD = reveal-md
MDPP = markdown-pp
FFMPEG = ffmpeg

# Try to find the correct markdown-pp command
MARKDOWN_PP_CMD := $(shell which markdown-pp 2>/dev/null || which markdownpp 2>/dev/null || echo "python -m markdownpp")

SLIDES ?= slides.mdpp
SLIDES_OUT ?= slides.md
MEDIA_DIR ?= media
SITE ?= _site
OPEN ?= xdg-open

.PHONY: all html clean videos

all: videos html

html: $(SITE)

$(SITE): $(SLIDES)
	@echo "=== Building slides ==="
	@echo "Current directory: $(PWD)"
	@echo "Working directory: $$(pwd)"
	@echo "Source file: $(SLIDES)"
	@echo "Output file: $(SLIDES_OUT)"
	@echo "Site directory: $(SITE)"
	@echo "Processing slides with markdown-pp..."
	@echo "Using command: $(MARKDOWN_PP_CMD)"
	@which $(MARKDOWN_PP_CMD) || (echo "Error: markdown-pp not found. Please install MarkdownPP." && exit 1)
	$(MARKDOWN_PP_CMD) $< -o $(SLIDES_OUT)
	@echo "Generated $(SLIDES_OUT), now creating static site..."
	@which $(RVMD) || (echo "Error: reveal-md not found. Please install reveal-md." && exit 1)
	$(RVMD) $(SLIDES_OUT) --static $@
	@echo "Static site generated successfully in $(SITE)/"
	@ls -la $(SITE)/ || echo "Warning: Could not list contents of $(SITE)/"

videos:
	@if [ -d "$(MEDIA_DIR)" ] && [ -n "$(TARGETS)" ]; then \
		for TARGET in $(TARGETS); do \
			$(FFMPEG) -framerate 0.5 -f image2 -y \
				-i "$(MEDIA_DIR)/$$TARGET/$$TARGET-%d.svg" -vf format=yuv420p $(MEDIA_DIR)/$$TARGET-generated.gif; \
		done; \
	else \
		echo "No media directory or targets defined, skipping video generation"; \
	fi

open: $(SITE)
	$(OPEN) $</index.html

clean:
	-rm -f $(MEDIA_DIR)/*-generated.gif
	-rm -f *~
	-rm -fr $(SITE) $(SLIDES_OUT)