const fs = require('fs/promises');

const minorTypes = ['feat'];

const increment = (version, target) => {
  const [major, minor, patch] = version.split('.').map(Number);
  if (target === 'major') {
    return `${major + 1}.${minor}.${patch}`;
  } else if (target === 'minor') {
    return `${major}.${minor + 1}.${patch}`;
  }
  return `${major}.${minor}.${patch + 1}`;
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
  if (message.toLowerCase().includes('breaking change')) {
    return 'major';
  }
  const [commitType] = message.split(/[^A-Za-z]/i);
  if (minorTypes.includes(commitType)) {
    return 'minor';
  }
  return 'patch';
};

(async () => {
  const [_node, _file, commitMessage] = process.argv;
  const target = parseTarget(commitMessage);
  await updateFiles(target);
})().catch(console.error);