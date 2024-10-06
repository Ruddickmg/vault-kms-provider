const fs = require('fs/promises');

const incrementVersion = (version, target) => {
  const [major, minor, patch] = version.split('.').map(Number);
  if (target === 'major') {
    return `${major + 1}.${minor}.${patch}`;
  } else if (target === 'minor') {
    return `${major}.${minor + 1}.${patch}`;
  }
  return `${major}.${minor}.${patch + 1}`;
};

const findAndReplace = (file, pattern, replace) => file.split('\n').map((line) => line.startsWith(pattern) ? replace(line) : line).join('\n');

(async () => {
  const helmFile = 'helm/Chart.yaml';
  const cargoFile = 'Cargo.toml';
  const target = 'patch';
  const [helmYaml, cargoToml]= await Promise.all([fs.readFile(helmFile), fs.readFile(cargoFile)]);
  const cargo = findAndReplace(cargoToml.toString(), 'version = "', line => {
    const [_, version] = line.split("\"");
    return `version = "${incrementVersion(version, target)}"`;
  });
  const version = findAndReplace(helmYaml.toString(), 'version: ', line => {
    const [_, version] = line.split(" ").filter((s) => s !== " ");
    return `version: ${incrementVersion(version, target)}`;
  });
  const helm = findAndReplace(version, 'appVersion: "', line => {
    const [_, version] = line.split("\"");
    return `appVersion: "${incrementVersion(version, target)}"`;
  });
  await Promise.all([fs.writeFile(cargoFile, cargo), fs.writeFile(helmFile, helm)]);
})().catch(console.error);