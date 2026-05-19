#!/usr/bin/env node
/**
 * npm postinstall: download the right native binary from GitHub Releases.
 *
 * Skipped silently in dev / CI when MIMO_SKIP_DOWNLOAD=1 is set.
 */

const fs = require('node:fs');
const path = require('node:path');
const https = require('node:https');
const { spawnSync } = require('node:child_process');
const crypto = require('node:crypto');

if (process.env.MIMO_SKIP_DOWNLOAD === '1') {
  console.log('[mimo-tui] MIMO_SKIP_DOWNLOAD set, skipping binary download');
  process.exit(0);
}

const pkg = require('./package.json');
const VERSION = `v${pkg.version}`;

const TARGETS = {
  'darwin-arm64': 'mimo-aarch64-apple-darwin',
  'linux-x64':    'mimo-x86_64-unknown-linux-gnu',
  'linux-arm64':  'mimo-aarch64-unknown-linux-gnu',
  'win32-x64':    'mimo-x86_64-pc-windows-msvc',
};

const key = `${process.platform}-${process.arch}`;
const targetName = TARGETS[key];
if (!targetName) {
  console.error(`[mimo-tui] unsupported platform: ${key}`);
  console.error(`[mimo-tui] supported: ${Object.keys(TARGETS).join(', ')}`);
  console.error('[mimo-tui] try `cargo install --git https://github.com/duolaAmengweb3/mimo-tui mimo-tui` instead');
  process.exit(0); // exit 0 so npm install doesn't fail
}

const isWindows = process.platform === 'win32';
const archiveExt = isWindows ? 'zip' : 'tar.gz';
const binName = isWindows ? 'mimo.exe' : 'mimo';
const url = `https://github.com/duolaAmengweb3/mimo-tui/releases/download/${VERSION}/${targetName}.${archiveExt}`;
const shaUrl = `${url}.sha256`;

const binDir = path.join(__dirname, 'bin');
fs.mkdirSync(binDir, { recursive: true });
const binPath = path.join(binDir, binName);

console.log(`[mimo-tui] downloading ${VERSION} (${key})`);

(async () => {
  try {
    const archivePath = path.join(binDir, `${targetName}.${archiveExt}`);
    await download(url, archivePath);

    // SHA verify (best-effort, skip if 404)
    try {
      const shaText = await downloadString(shaUrl);
      const expectedSha = shaText.split(/\s+/)[0];
      const actualSha = sha256(archivePath);
      if (expectedSha && actualSha !== expectedSha) {
        throw new Error(`sha256 mismatch: got ${actualSha}, expected ${expectedSha}`);
      }
    } catch (e) {
      console.warn(`[mimo-tui] (skipping sha verify: ${e.message})`);
    }

    if (isWindows) {
      // unzip via PowerShell
      const r = spawnSync('powershell.exe', [
        '-NoProfile', '-Command',
        `Expand-Archive -LiteralPath '${archivePath}' -DestinationPath '${binDir}' -Force`,
      ], { stdio: 'inherit' });
      if (r.status !== 0) throw new Error('unzip failed');
    } else {
      const r = spawnSync('tar', ['xzf', archivePath, '-C', binDir], { stdio: 'inherit' });
      if (r.status !== 0) throw new Error('untar failed');
    }

    // The archive expands as `mimo-<target>/mimo[.exe]` — flatten.
    const expandedDir = path.join(binDir, targetName);
    if (fs.existsSync(expandedDir)) {
      const inner = path.join(expandedDir, binName);
      if (fs.existsSync(inner)) {
        fs.renameSync(inner, binPath);
      }
      fs.rmSync(expandedDir, { recursive: true, force: true });
    }

    fs.unlinkSync(archivePath);

    if (!isWindows) {
      fs.chmodSync(binPath, 0o755);
    }

    console.log(`[mimo-tui] installed → ${binPath}`);
  } catch (err) {
    console.error(`[mimo-tui] download failed: ${err.message}`);
    console.error(`[mimo-tui] you can grab the binary manually:`);
    console.error(`           ${url}`);
    console.error(`           or run: cargo install --git https://github.com/duolaAmengweb3/mimo-tui mimo-tui`);
    process.exit(0); // don't break npm install
  }
})();

function download(url, dest, redirects = 0) {
  return new Promise((resolve, reject) => {
    if (redirects > 6) return reject(new Error('too many redirects'));
    const file = fs.createWriteStream(dest);
    https
      .get(url, (res) => {
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          file.close();
          fs.unlinkSync(dest);
          return resolve(download(res.headers.location, dest, redirects + 1));
        }
        if (res.statusCode !== 200) {
          file.close();
          fs.unlinkSync(dest);
          return reject(new Error(`HTTP ${res.statusCode} from ${url}`));
        }
        res.pipe(file);
        file.on('finish', () => file.close(resolve));
      })
      .on('error', (err) => {
        try { fs.unlinkSync(dest); } catch (_) {}
        reject(err);
      });
  });
}

function downloadString(url, redirects = 0) {
  return new Promise((resolve, reject) => {
    if (redirects > 6) return reject(new Error('too many redirects'));
    https
      .get(url, (res) => {
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          return resolve(downloadString(res.headers.location, redirects + 1));
        }
        if (res.statusCode !== 200) {
          return reject(new Error(`HTTP ${res.statusCode}`));
        }
        let data = '';
        res.on('data', (c) => (data += c));
        res.on('end', () => resolve(data));
      })
      .on('error', reject);
  });
}

function sha256(filePath) {
  const hash = crypto.createHash('sha256');
  hash.update(fs.readFileSync(filePath));
  return hash.digest('hex');
}
