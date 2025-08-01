import chokidar from "npm:chokidar";
import { exec }from "node:child_process";
import path from "node:path";

// Define the directory to watch
const watchDir = path.join(process.cwd(), "./wasm/src/");

// Initialize watcher
const watcher = chokidar.watch(watchDir, {
  persistent: true,
  ignoreInitial: false,
  ignored: /(^|[\/\\])\../, // Ignore dot files
});

// Function to run the build command
function runBuild() {
  console.log("File change detected, running pnpm build:wasm...");
  exec("pnpm build:wasm", (error, stdout, stderr) => {
    if (error) {
      console.error(`Build error: ${error.message}`);
      return;
    }
    if (stderr) {
      console.error(`Build stderr: ${stderr}`);
      return;
    }
    console.log(`Build output: ${stdout}`);
  });
}

watcher
  .on("add", path => {
    console.log(`File ${path} has been added`);
    runBuild();
  })
  .on("change", path => {
    console.log(`File ${path} has been changed`);
    runBuild();
  })
  .on("unlink", path => {
    console.log(`File ${path} has been removed`);
    runBuild();
  })
  .on("error", error => console.error(`Watcher error: ${error}`));

console.log(`Watching for file changes in ${watchDir}`);
