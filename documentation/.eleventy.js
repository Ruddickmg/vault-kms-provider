import syntaxHighlight from '@11ty/eleventy-plugin-syntaxhighlight';
import markdown from 'markdown-it';
import prism from 'markdown-it-prism';
import markdownItGitHubAlerts from 'markdown-it-github-alerts';
import { Liquid } from 'liquidjs';

export default function (config) {
  const md = markdown({
    html: true,
    breaks: true,
    linkify: true,
  })
    .use(markdownItGitHubAlerts, {
      markers: "*"
    })
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
