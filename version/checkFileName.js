(() => {
  const [_node, _file, message] = process.argv;
  const types = ['chore', 'patch', 'fix', 'feat'];
  const [commitType] = message?.split(/[^A-Za-z]/i) || [];
  if (!types.includes(commitType) && !message?.toLowerCase().includes('breaking change')) {
    throw new Error(`Invalid commit name, must include "BREAKING CHANGE" or one of the following prefixes: ${types.map(s => `"${s}"`).join(',')}`);
  }
})();