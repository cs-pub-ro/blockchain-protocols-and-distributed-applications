#!/usr/bin/env python3
import os
import sys
import yaml


def flatten_nav(nav_section):
    items = []
    for entry in nav_section:
        if isinstance(entry, dict):
            for _title, target in entry.items():
                if isinstance(target, list):
                    items.extend(flatten_nav(target))
                else:
                    items.append(target)
        else:
            items.append(entry)
    return items


def main():
    repo_root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    mkdocs_path = os.path.join(repo_root, "mkdocs.yml")
    with open(mkdocs_path, "r", encoding="utf-8") as f:
        cfg = yaml.safe_load(f)
    docs_dir = cfg.get("docs_dir", "docs")
    nav = cfg.get("nav", [])
    files = flatten_nav(nav)
    for rel in files:
        if not isinstance(rel, str):
            continue
        print(os.path.join("book", rel))


if __name__ == "__main__":
    sys.exit(main())


