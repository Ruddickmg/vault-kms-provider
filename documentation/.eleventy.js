const syntaxHighlight = require('@11ty/eleventy-plugin-syntaxhighlight');
const markDown = require('markdown-it');
const { Liquid } = require('liquidjs');

module.exports = function (config) {
  config.addPlugin(syntaxHighlight);
  config.setLibrary('liquid', new Liquid({
    extname: '.liquid',
    dynamicPartials: false,
    strictFilters: false, // renamed from `strict_filters` in Eleventy 1.0
    root: ['_includes'],
  }));
  config.setLibrary('md', markDown({
    html: true,
    breaks: true,
    linkify: true,
  }));
  return {
    dir: {
      input: "src",
      output: "docs",
      includes: "_includes",
    }
  }
};
