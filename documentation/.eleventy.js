const syntaxHighlight = require('@11ty/eleventy-plugin-syntaxhighlight');
const markdown = require('markdown-it');
const prism = require('markdown-it-prism');
const markdownItAttrs = require('markdown-it-attrs');
const githubAlerts = require('markdown-it-github-alerts');
const { Liquid } = require('liquidjs');

module.exports = function (config) {
  const md = markdown({
    html: true,
    breaks: true,
    linkify: true,
  });
  md.use(prism);
  md.use(markdownItAttrs);
  md.use(githubAlerts);
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
