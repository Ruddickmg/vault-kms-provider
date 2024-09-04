---
tags:
  - documentation
override:tags: ["documentation"]
eleventyExcludeFromCollections:
  - configuration
---

# Configuration

{% for configuration in collections.configuration %}
{{ configuration.content }}
{% endfor %}