const { spawn } = require('child_process')

async function getAvailablePort() {
    // * Simple dynamic port allocation (can improve with OS check)
    return Math.floor(Math.random() * (10101 - 10000)) + 10000;
}

async function runCommand(command, args) {
  return new Promise((resolve, reject) => {
    const proc = spawn(command, args, { shell: true }); //* shell:true makes it safer

    proc.stdout.on("data", (data) => {
      console.log(data.toString());
    });

    proc.stderr.on("data", (data) => {
      console.error(data.toString());
    });

    proc.on("close", (code) => {
      if (code === 0) {
        console.log('COMMAND EXECUTED SUCCESSFULLY...')
        resolve();
      } else {
        reject(new Error(`Command failed with code ${code}`));
      }
    });

    proc.on("error", (err) => {
      reject(err);
    });
  });
}

module.exports = { getAvailablePort, runCommand }
