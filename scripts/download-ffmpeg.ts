import ffbinaries from 'ffbinaries';
import path from 'node:path';
import fs from 'node:fs/promises';
import { chmod } from 'node:fs/promises';

// Define the output directory
const dest = path.join(import.meta.dirname, '../src-tauri/binaries');

// Mapping: ffbinaries platform code -> Tauri target triple filename
const platformMap: Record<string, string> = {
  'windows-64':  'ffmpeg-x86_64-pc-windows-msvc.exe',
  'osx-64':      'ffmpeg-x86_64-apple-darwin',
  'linux-64':    'ffmpeg-x86_64-unknown-linux-gnu',
  'linux-arm64': 'ffmpeg-aarch64-unknown-linux-gnu',
};

async function main() {
  await fs.mkdir(dest, { recursive: true });
  console.log('🚀 Starting FFmpeg download...');

  // 1. Download for each platform in parallel
  const downloadPromises = Object.entries(platformMap).map(async ([platformCode, targetFilename]) => {
    return downloadAndRename(platformCode as ffbinaries.Platform, targetFilename);
  });

  await Promise.all(downloadPromises);

  // 2. Handle M1/M2/M3 Mac (Copy Intel to ARM)
  await copyIntelToArm();

  console.log('🎉 Done! FFmpeg binaries are ready in src-tauri/binaries/');
}

async function downloadAndRename(platform: ffbinaries.Platform, targetFilename: string) {
  const tempDir = path.join(dest, `_temp_${platform}`);
  await fs.mkdir(tempDir, { recursive: true });

  return new Promise<void>((resolve, reject) => {
    ffbinaries.downloadBinaries(['ffmpeg'], { 
      destination: tempDir, 
      platform: platform 
    }, async (err) => {
      if (err) {
        await fs.rm(tempDir, { recursive: true, force: true }).catch(() => {});
        return reject(err);
      }
      
      try {
        // [CRITICAL CHANGE] Hunt for the actual binary file recursively
        const binaryName = platform.startsWith('windows') ? 'ffmpeg.exe' : 'ffmpeg';
        const foundBinaryPath = await findFileRecursively(tempDir, binaryName);

        if (!foundBinaryPath) {
          throw new Error(`Binary ${binaryName} not found in extracted folder for ${platform}`);
        }

        const finalPath = path.join(dest, targetFilename);

        // Nuke the destination if it exists (file OR folder)
        await fs.rm(finalPath, { recursive: true, force: true });

        // Move the specific file found, not the folder
        await fs.rename(foundBinaryPath, finalPath);
        console.log(`   • Ready: ${targetFilename}`);

        if (!targetFilename.endsWith('.exe')) {
          await chmod(finalPath, 0o755);
        }
      } catch (e) {
        reject(new Error(`Failed to process ${platform}: ${e}`));
      } finally {
        // Cleanup temp folder
        await fs.rm(tempDir, { recursive: true, force: true }).catch(() => {});
      }
      
      resolve();
    });
  });
}

// Recursive helper to find the file even if nested in subfolders
async function findFileRecursively(dir: string, filename: string): Promise<string | null> {
  const entries = await fs.readdir(dir, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    
    // If we found the specific file we want
    if (entry.isFile() && entry.name === filename) {
      return fullPath;
    }
    
    // If directory, dive in
    if (entry.isDirectory()) {
      const found = await findFileRecursively(fullPath, filename);
      if (found) return found;
    }
  }
  return null;
}

async function copyIntelToArm() {
  const intelMacPath = path.join(dest, 'ffmpeg-x86_64-apple-darwin');
  const armMacPath = path.join(dest, 'ffmpeg-aarch64-apple-darwin');

  try {
    try {
      await fs.access(armMacPath);
      return; 
    } catch { /* proceed */ }

    await fs.access(intelMacPath);
    await fs.rm(armMacPath, { recursive: true, force: true }); // Ensure clean target
    await fs.copyFile(intelMacPath, armMacPath);
    console.log(`   • Copied Intel binary to ${path.basename(armMacPath)} (Runs via Rosetta)`);
  } catch (e) {
    // Intel file likely missing
  }
}

main().catch(console.error);