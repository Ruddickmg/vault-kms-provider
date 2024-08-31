const syntaxHighlight = require('@11ty/eleventy-plugin-syntaxhighlight');
const markdown = require('markdown-it');
const prism = require('markdown-it-prism');

const { Liquid } = require('liquidjs');

module.exports = function (config) {
  const md = markdown({
    html: true,
    breaks: true,
    linkify: true,
  });
  md.use(prism);
  config.addPlugin(syntaxHighlight);
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
