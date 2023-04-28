---
layout: default
title: Contributing
nav_order: 2
permalink: /contributing
has_children: false
---

# Contributing to Ted

## Contributors
<ul class="list-style-none">
{% for contributor in site.github.contributors %}
  <li class="d-inline-block mr-1">
    <a href="{{ contributor.html_url }}"><img src="{{ contributor.avatar_url }}" width="32" height="32" alt="{{ contributor.login }}"></a>
  </li>
{% endfor %}
