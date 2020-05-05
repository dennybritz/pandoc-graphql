
# BlogAPI

Turn your content into a flexible API, then serve it as a single page, blog, or whatever else you want. This library monitors your files, converts them, and then serves a GraphQL API. For example, you 

## Introduction

This project was build out of frustration with static site generators such as Jekyll or Hugo. I've used most of them, and while they are convenient, I felt locked-into my choice, because the content was tightly coupled with the design and layout of the pages. I also couldn't write content in any form I wanted, such as latex.

## QuickStart




## Supported Front Matter

Inspired by

- https://gohugo.io/content-management/front-matter/
- https://jekyllrb.com/docs/front-matter/

```yaml
title: A post in markdown
date: "2020-04-10"

description: |
  A longer preview 

slug: "a-post-in-markdown"
draft: true
tags:
  - Machine Learning
outputs:
  - html
  - pdf
```