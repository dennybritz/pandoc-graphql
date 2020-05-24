
# pandoc-graphql

Turns your local documents into a GraphQL API using pandoc.

## Why would you ever want to do that?

- Write content in any format, such as Markdown or Jupyter, query it, and transform it to  output formats like HTML, PDF and RSS using an API.
- Cleanly separate your content from the consuming application(s). For example, your blog posts or notes may live in a content-only Github repository and are then served to multiple websites or static site generators.
- Built-in support for academic citations via `pandoc-citeproc`

## Introduction

This project was build out of frustration with static site generators such as Jekyll or Hugo. I've used most of them, and while they are convenient, I felt locked-into my choice because the content was coupled with the design and layout of the pages. I also couldn't write content in any form I wanted, such as latex.

## QuickStart

```
cargo run -- serve --path index.yaml
```

## Content Definition

You define your content using one or more YAML files. Each YAML documents is a [pandoc defaults file](https://pandoc.org/MANUAL.html#default-files) that is passed to pandoc. This file customizes how pandoc builds your content. For example, a blog would have one YAML document per blog post. The following example shows a file declaring two blog posts (one is a draft) and an about page. In this case, all three documents are defined in a single YAML stream, but you could also split them across multiple files.

```yaml
metadata:
  id: "about"
  title: About Me
  author: ["Denny Britz"]
input-file: about.md
---
metadata:
  id: "writing-in-markdown"
  collections: ["drafts"]
  title: A blog post written in Markdown
  author: ["Denny Britz"]
  date: "2020-01-01"
  assets:
    - "writing-in-markdown"
input-files: 
- writing-in-markdown/part1.md
- writing-in-markdown/part2.md  
---
metadata:
  id: "writing-in-jupyter"
  collections: ["posts"]
  title: A blog post written in Jupyter
  author: ["Denny Britz"]
  date: "2020-01-01"
input-file: writing-in-jupyter/notebook.ipynb
```

## Metadata

The metadata block contains arbitrary metadata. Pandoc uses some of this metadata to customize the document generation. You can find a list of pandoc's metadata variables [here](https://pandoc.org/MANUAL.html#variables). In addition, the following metadata variables are used directly for the API (many of these overlap with pandoc):

```yaml
# Each document should have a unique ID
id: "my-post"

title: Post Title

# A document belongs to a set of collections
# This allows you to query for specific types of documents, e.g. "draft" posts in your frontend application
collections: 
  - "Posts"

author:
  - Denny Britz

# Must be in YYYY-MM-DD format
date: "2020-01-01"

description: |
  A longer preview of your content

# Arbitrary tags
tags:
- JAX
- Machine Learning

# A bibliography file used to generate citations
# anything supported by pandoc-citeproc is valid here
# See https://github.com/jgm/pandoc-citeproc/blob/master/man/pandoc-citeproc.1.md
bibliography: references.bib
```

## Assets

From within your content, you can link assets using relative paths such as `image.jpg`, `assets/image.jpg` or `../assets/image.jpg`. However, pandoc assumes all assets paths are relative to the directory pandoc is called from, not the path of the file that is being processed. Thus, you must use the [`--resource-path`](https://pandoc.org/MANUAL.html#option--resource-path) option in your config to add additional paths.

Example:

The directory structure is:

```
/
/my-post/content.md
/my-post/image.jpg
```

And

- You refer to `./image.jpg` from `content.md`
- Pandoc is started from the directory root `/`


This means pandoc will look for `image.jpg` only in `/` and you must add `resource-path: ["my-post"]` to your document configuration. You can find examples for this in this repository.


## Misc

Build Docker images

```
docker build -t dennybritz/pandoc-graphql .
```
