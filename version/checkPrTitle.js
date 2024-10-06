(() => {
  const [_node, _file, message] = process.argv;
  const types = ['chore', 'patch', 'fix', 'feat'];
  const [commitType] = message?.trim().split(/[^A-Za-z]/i) || [];
  if (!types.includes(commitType) && !message?.toLowerCase().includes('breaking change')) {
    console.log("message", message);
    throw new Error(`Invalid PR title. Title must include "BREAKING CHANGE" or one of the following prefixes: ${types.map(s => `"${s}"`).join(',')}`);
  }
})();