import syntaxHighlight from "@11ty/eleventy-plugin-syntaxhighlight";
import markDown from "markdown-it"

export default function (config) {
  config.addPlugin(syntaxHighlight);
  config.setLibrary("md", markDown({
    html: true,
    breaks: true,
    linkify: true,
  }));
};
