#!/usr/bin/env node
/**
 * Shim that execs the platform-native `mimo` binary downloaded by install.js.
 */

const path = require('node:path');
const fs = require('node:fs');
const { spawn } = require('node:child_process');

const isWindows = process.platform === 'win32';
const binName = isWindows ? 'mimo.exe' : 'mimo';
const binPath = path.join(__dirname, binName);

if (!fs.existsSync(binPath)) {
  console.error(`[mimo-tui] native binary not found at ${binPath}.`);
  console.error('[mimo-tui] re-install: `npm install -g mimo-tui --force`');
  console.error('[mimo-tui] or grab the binary directly from');
  console.error('           https://github.com/duolaAmengweb3/mimo-tui/releases/latest');
  process.exit(1);
}

const child = spawn(binPath, process.argv.slice(2), {
  stdio: 'inherit',
  windowsHide: false,
});
child.on('exit', (code, sig) => {
  if (sig) process.kill(process.pid, sig);
  else process.exit(code ?? 0);
});
child.on('error', (err) => {
  console.error(`[mimo-tui] failed to spawn ${binPath}: ${err.message}`);
  process.exit(1);
});
