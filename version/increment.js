const fs = require('fs/promises');

const Increment = Object.freeze({
  Major: 'major',
  Minor: 'minor',
  Patch: 'patch',
});

const nonIncrementalTypes = ['build', 'ci', 'docs', 'perf', 'test'];
const patchTypes = ['fix', 'chore', 'patch', 'style', 'refactor'];
const minorTypes = ['feat'];
const allTypes = patchTypes.concat(minorTypes).concat(nonIncrementalTypes);

const increment = (version, target) => {
  const [major, minor, patch] = version.split('.').map(Number);
  if (target === Increment.Major) {
    return `${major + 1}.0.0`;
  } else if (target === Increment.Minor) {
    return `${major}.${minor + 1}.0`;
  } else if (target === Increment.Patch) {
    return `${major}.${minor}.${patch + 1}`;
  }
  return version;
};

const findAndReplace = (file, pattern, replace) => file.split('\n').map((line) => line.startsWith(pattern) ? replace(line) : line).join('\n');

const updateFiles = async (target) => {
  const helmFile = 'helm/Chart.yaml';
  const cargoFile = 'Cargo.toml';
  const [helmYaml, cargoToml] = await Promise.all([fs.readFile(helmFile), fs.readFile(cargoFile)]);
  const cargo = findAndReplace(cargoToml.toString(), 'version = "', line => {
    const [_, version] = line.split("\"");
    return `version = "${increment(version, target)}"`;
  });
  const version = findAndReplace(helmYaml.toString(), 'version: ', line => {
    const [_, version] = line.split(" ").filter((s) => s !== " ");
    return `version: ${increment(version, target)}`;
  });
  const helm = findAndReplace(version, 'appVersion: "', line => {
    const [_, version] = line.split("\"");
    return `appVersion: "${increment(version, target)}"`;
  });
  await Promise.all([fs.writeFile(cargoFile, cargo), fs.writeFile(helmFile, helm)]);
}

const parseTarget = (message) => {
  const [commitType] = message.split(/[^A-Za-z]/i);
  if (!allTypes.includes(commitType)) throw new Error(`Invalid commit type, must be one of: ${allTypes.map(s => `"${s}"`).join(', ')}`)
  if (message.toLowerCase().includes('breaking change')) return Increment.Major;
  if (minorTypes.includes(commitType)) return Increment.Minor;
  if (patchTypes.includes(commitType)) return Increment.Patch;
  return 'ignore';
};

(async () => {
  const [_node, _file, commitMessage] = process.argv;
  const target = parseTarget(commitMessage);
  await updateFiles(target);
})().catch(console.error);