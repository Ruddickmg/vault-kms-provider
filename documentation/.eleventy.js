import syntaxHighlight from '@11ty/eleventy-plugin-syntaxhighlight';
import prism from 'markdown-it-prism';
import { Liquid } from 'liquidjs';
import MarkdownIt from 'markdown-it'
import { default as MarkdownItGitHubAlerts } from 'markdown-it-github-alerts'

export default function (config) {
  const md = MarkdownIt({
    html: true,
    breaks: true,
    linkify: true,
  })
    .use(MarkdownItGitHubAlerts)
    .use(prism);
  config.addPlugin(syntaxHighlight);
  config.setBrowserSyncConfig({
    files: './docs/css/**/*.css'
  });
  config.setLibrary('liquid', new Liquid({
    extname: '.liquid',
    dynamicPartials: false,
    strictFilters: false, // renamed from `strict_filters` in Eleventy 1.0
    root: ['_includes'],
  }));
  config.setLibrary('md', md);
  return {
    dir: {
      input: "src",
      output: "docs",
      includes: "_includes",
    }
  }
};
